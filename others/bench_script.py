import os

def escape_quotes(content):
    return content.replace('\\', '\\\\').replace('"', '\\"')

def generate_constants(directory, output_file):
    with open(output_file, 'w', encoding='utf-8') as outfile:
        for filename in os.listdir(directory):
            if filename == "benches.rs":
                continue
            filepath = os.path.join(directory, filename)
            if os.path.isfile(filepath):
                with open(filepath, 'r', encoding='utf-8') as file:
                    content = file.read()
                    escaped_content = escape_quotes(content)
                    const_name = filename.replace(".", "_").upper()
                    outfile.write(f'pub const {const_name}: &str = "{escaped_content}";\n')

# Specifica qui la directory che vuoi usare e il file di output
directory_path = "./benches"
output_file = "./src/bench.rs"
generate_constants(directory_path, output_file)