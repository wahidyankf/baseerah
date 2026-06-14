module OseBe.Infrastructure.OpenRouterClient

open System
open System.Net.Http
open System.Net.Http.Headers
open System.Text
open System.Text.Json
open System.Threading.Tasks

/// OpenRouter LLM client configuration, sourced from the OSE_BE_OPENROUTER_*
/// environment variables. The API key is a secret: it is read from the
/// environment only and is never committed (placeholder in .env.example).
type OpenRouterConfig =
    { ApiKey: string
      Model: string
      BaseUrl: string }

/// Default OpenRouter model identifier.
[<Literal>]
let DefaultModel = "openrouter/auto"

/// Default OpenRouter API base URL.
[<Literal>]
let DefaultBaseUrl = "https://openrouter.ai/api/v1"

let private envOr (name: string) (fallback: string) : string =
    match Environment.GetEnvironmentVariable(name) with
    | null
    | "" -> fallback
    | value -> value

/// Loads the OpenRouter configuration from the OSE_BE_OPENROUTER_* env vars.
/// A missing API key yields an empty string (LLM calls are disabled until a key
/// is provided); the model and base URL fall back to documented defaults.
let loadConfig () : OpenRouterConfig =
    { ApiKey = envOr "OSE_BE_OPENROUTER_API_KEY" ""
      Model = envOr "OSE_BE_OPENROUTER_MODEL" DefaultModel
      BaseUrl = envOr "OSE_BE_OPENROUTER_BASE_URL" DefaultBaseUrl }

/// Whether the client is configured with an API key and can issue live calls.
let isConfigured (config: OpenRouterConfig) : bool =
    not (String.IsNullOrWhiteSpace config.ApiKey)

/// Requests a chat completion from OpenRouter for the given prompt.
///
/// Returns Ok with the model's text response, or Error with a diagnostic
/// message. When no API key is configured the call short-circuits with an Error
/// rather than issuing an unauthenticated request (no live calls in tests).
let complete (config: OpenRouterConfig) (prompt: string) : Task<Result<string, string>> =
    task {
        if not (isConfigured config) then
            return Error "OpenRouter API key is not configured"
        else
            try
                use client = new HttpClient()
                client.BaseAddress <- Uri(config.BaseUrl.TrimEnd('/') + "/")
                client.DefaultRequestHeaders.Authorization <- AuthenticationHeaderValue("Bearer", config.ApiKey)

                let payload =
                    JsonSerializer.Serialize(
                        {| model = config.Model
                           messages = [| {| role = "user"; content = prompt |} |] |}
                    )

                use content = new StringContent(payload, Encoding.UTF8, "application/json")
                use! response = client.PostAsync("chat/completions", content)
                let! body = response.Content.ReadAsStringAsync()

                if response.IsSuccessStatusCode then
                    return Ok body
                else
                    return Error(sprintf "OpenRouter returned %d: %s" (int response.StatusCode) body)
            with ex ->
                return Error(sprintf "OpenRouter request failed: %s" ex.Message)
    }
