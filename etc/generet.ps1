$filePath = "./mime_types.txt"  # Change this to your actual file path

# Read and parse the dictionary-like file
$content = Get-Content -Raw -Path $filePath

# Start Rust HashMap declaration
$rustCode = @"
use std::collections::HashMap;

fn main() {
    let mut mime_types: HashMap<String, String> = HashMap::new();
"@

# Process each line and generate Rust insert statements
foreach ($line in $content -split "`n") {
    if ($line -match '"(.+?)"\s*,\s*"(.+?)"') {
        $key = $matches[1]
        $value = $matches[2]
    $rustCode += "    mime_types.insert(`"$key`".to_string(), `"$value`".to_string());`n"
    }
}

# Close the Rust main function
$rustCode += "}";

# Output the generated Rust code
$rustCode | Out-File -Encoding utf8 -FilePath "output.rs"
Write-Output "Rust code has been generated in output.rs"
