#!/bin/bash

# Define the input and output directories
INPUT_DIR=./dbml
OUTPUT_DIR=./src/models

# Create the output directory if it doesn't exist
mkdir -p $OUTPUT_DIR

# Generate the Rust file for the schema
echo "Generating Rust file for schema"
dbml2sqlx $INPUT_DIR/schema.dbml | sqlx-introspect > $OUTPUT_DIR/schema.rs

# Generate the Rust file for each model
for model in Auth Games Rounds Moves Points Times PasswordResetRequests; do
  echo "Generating Rust file for $model"
  dbml2sqlx $INPUT_DIR/$model.dbml | sqlx-introspect > $OUTPUT_DIR/$model.rs
done
