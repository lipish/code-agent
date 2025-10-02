#!/bin/bash

# Quick verification that test setup is working
echo "🔍 Verifying Test Setup"
echo "======================"

# Check if all required files exist
echo "📁 Checking test files..."

required_files=(
    "test_agent.sh"
    "test_detailed.sh"
    "demo.sh"
    "run_tests.sh"
    "test_file"
    "test_script"
    "README.md"
)

missing_files=()
for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
        missing_files+=("$file")
    fi
done

if [ ${#missing_files[@]} -eq 0 ]; then
    echo ""
    echo "✅ All test files are present!"
else
    echo ""
    echo "❌ Missing files: ${missing_files[*]}"
    exit 1
fi

# Check if scripts are executable
echo ""
echo "🔧 Checking script permissions..."
for script in *.sh; do
    if [ -x "$script" ]; then
        echo "✅ $script is executable"
    else
        echo "❌ $script is not executable"
    fi
done

# Check if we can run a simple cargo command from project root
echo ""
echo "🚀 Testing cargo build..."
cd .. 2>/dev/null
if cargo check --quiet; then
    echo "✅ Project builds successfully"
else
    echo "❌ Project build failed"
    exit 1
fi

# Check if test_script works
echo ""
echo "🧪 Testing test_script execution..."
cd test 2>/dev/null
if ./test_script > /dev/null 2>&1; then
    echo "✅ test_script executes successfully"
else
    echo "❌ test_script execution failed"
fi

echo ""
echo "🎉 Test setup verification complete!"
echo "=================================="
echo "You can now run the full test suite with: ./run_tests.sh"