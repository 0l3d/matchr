# matchr
`matchr` is a lightweight and efficient fuzzy matching library written in Rust.  
It helps you score and sort candidate strings by how well they match a query,  
with a focus on CLI tools, search features, and quick approximate matching.

## Features
- **Position-weighted scoring** - Characters matched earlier in the candidate string get higher scores
- **Consecutive character bonus** - Adjacent matched characters earn bonus points
- **Exact match detection** - Perfect matches always score 100
- **Subsequence validation** - Only valid subsequences are scored
- **Batch matching** - Score and sort multiple candidates at once
- **Zero dependencies** - Pure Rust implementation
- **Simple API** - Just two main functions to get started

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
matchr = "0.2.5"
```

## Usage
### Basic Scoring
```rust
use matchr::score;

let query = "fefe";
let candidate = "feature";
let match_score = score(query, candidate);
println!("Score: {}", match_score);
```

### Matching Multiple Items
```rust
use matchr::match_items;

let query = "xb";
let candidates = ["xbps-install", "xbps-remove", "xbps-query", "grep", "find"];
let results = match_items(query, &candidates);

for (item, score) in results {
    println!("{} => score: {}", item, score);
}
// Output (sorted by score):
// xbps-install => score: 90
// xbps-remove => score: 85
// xbps-query => score: 82
// ...
```

## API Reference
### Functions
#### `score(query: &str, candi: &str) -> usize`
Scores how well a candidate string matches the query based on subsequence matching.

**Parameters:**
- `query` - The search query string
- `candi` - The candidate string to match against

**Returns:** A score between 0 and 100, where higher means better match

**Scoring Logic:**
- Characters matched earlier in the candidate get higher weight: `10 - position`
- Consecutive matched characters earn a bonus: `score / 10`
- Final score is normalized to 0-100 range
- Exact matches always return 100
- Non-subsequences return 0

#### `match_items<'a>(query: &str, items: &[&'a str]) -> Vec<(&'a str, usize)>`
Matches multiple items against a query and returns them sorted by score.

**Parameters:**
- `query` - The search query string
- `items` - Slice of candidate strings

**Returns:** Vector of `(item, score)` tuples, sorted by descending score

## Examples
### CLI Tool Integration
```rust
use matchr::match_items;

fn search_commands(query: &str) -> Vec<String> {
    let commands = ["git commit", "git push", "git pull", "git status", "grep"];
    let results = match_items(query, &commands);
    
    results
        .into_iter()
        .filter(|(_, score)| *score > 10)
        .map(|(cmd, _)| cmd.to_string())
        .collect()
}

let matches = search_commands("git");
// Returns commands containing "git" as subsequence
```

### File Search
```rust
use matchr::match_items;

let query = "cfg";
let files = ["config.toml", "Cargo.toml", "src/cfg.rs", "README.md"];
let results = match_items(query, &files);

for (file, score) in results.iter().take(3) {
    if *score > 0 {
        println!("📁 {} (score: {})", file, score);
    }
}
```

### Package Manager Search
```rust
use matchr::match_items;

let query = "xb";
let packages = [
    "xbps-install", "xbps-remove", "xbps-query", 
    "bash", "zsh", "fish", "curl", "wget"
];
let results = match_items(query, &packages);

// xbps-* packages will score highest due to early position matches
```

## Performance
`matchr` is designed to be fast and memory-efficient:
- No heap allocations during scoring
- O(n×m) time complexity where n = query length, m = candidate length
- Suitable for interactive applications and real-time search
- Position-weighted algorithm provides intuitive results

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
Licensed under either of
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---
Created by **oled**
