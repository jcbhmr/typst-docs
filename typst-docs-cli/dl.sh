#!/bin/bash

# Create a directory to store downloaded files
mkdir -p public

# Loop through each line in the input file
while IFS= read -r url; do
    # Remove query string from URL
    url_without_query=$(echo "$url" | cut -d'?' -f1)

    # Determine the directory to save the file based on the URL
    if [[ $url_without_query == *"assets/"* ]]; then
        directory="public/assets"
    elif [[ $url_without_query == *"scripts/"* ]]; then
        directory="public/scripts"
    elif [[ $url_without_query == *"styles/"* ]]; then
        directory="public/styles"
    else
        directory="public"
    fi

    # Create the directory if it doesn't exist
    mkdir -p "$directory"

    # Extract filename from URL
    filename=$(basename "$url_without_query")

    # Download the file
    echo "Downloading $url_without_query to $directory/$filename"
    curl -sS "$url_without_query" -o "$directory/$filename"
done < input.txt
