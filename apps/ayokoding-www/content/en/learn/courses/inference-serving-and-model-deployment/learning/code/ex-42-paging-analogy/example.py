"""Example 42: The Paging Analogy."""

OS_TO_KV_CACHE_MAPPING = {  # => co-09: the KV cache borrows this vocabulary directly from OS virtual memory
    "physical page frame": "cache block",  # => the fixed-size unit that actually gets allocated
    "virtual address space": "a request's logical KV cache",  # => what the request THINKS it has
    "page table": "block table (maps logical block -> physical block)",  # => the indirection layer
    "internal fragmentation": "at most ONE partially-full block per request",  # => co-09: the bounded waste
    "external fragmentation": "eliminated -- any free block fits any request's next block",  # => uniform size
}  # => five OS concepts, five direct KV-cache counterparts -- the analogy is nearly one-to-one

for os_term, cache_term in OS_TO_KV_CACHE_MAPPING.items():  # => confirm every OS concept has a cache counterpart
    print(f"{os_term} -> {cache_term}")  # => prints all five mappings, one per line

assert len(OS_TO_KV_CACHE_MAPPING) == 5  # => co-09: five load-bearing concepts, all present
assert OS_TO_KV_CACHE_MAPPING["external fragmentation"].startswith("eliminated")  # => co-09's key promise
# => knowing this vocabulary makes vLLM/TGI documentation legible without re-deriving it from scratch
print("ex-42 OK")  # => a self-check marker confirming all five OS-to-cache mappings are present
