"""Example 17: Tokens vs Characters."""


def naive_word_tokenize(text: str) -> list[str]:  # => co-01: a stand-in for a real subword tokenizer
    return text.split()  # => whitespace-split -- real tokenizers split sub-word, but the LESSON transfers


english = "The quick brown fox jumps."  # => short in tokens, short in characters too
url = "https://example.com/a/very/long/path/segment/that/keeps/going"  # => ONE "word", many characters
# => this pair is deliberately adversarial: character count and token count DISAGREE on which is bigger

english_tokens = naive_word_tokenize(english)  # => 5 whitespace-separated words
url_tokens = naive_word_tokenize(url)  # => 1 whitespace-separated "word" -- the url has no spaces
print(len(english), len(url))  # => Output: 26 61 -- url has over TWICE the characters
print(len(english_tokens), len(url_tokens))  # => Output: 5 1 -- but FEWER tokens, not more

assert len(url) > len(english)  # => character count says the url is "bigger"
assert len(url_tokens) < len(english_tokens)  # => co-01: token count -- the real cost driver -- disagrees
print("ex-17 OK")  # => a self-check marker confirming characters and tokens disagreed as predicted
