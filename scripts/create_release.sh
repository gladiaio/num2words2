#!/bin/bash

# Script to create a new release
# Usage: ./scripts/create_release.sh [version]
# Example: ./scripts/create_release.sh 1.0.0

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get version from command line or prompt
if [ -z "$1" ]; then
    echo -e "${YELLOW}Enter version number (e.g., 1.0.0):${NC}"
    read VERSION
else
    VERSION=$1
fi

# Validate version format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}Error: Invalid version format. Use semantic versioning (e.g., 1.0.0)${NC}"
    exit 1
fi

TAG="v$VERSION"

echo -e "${GREEN}Creating release $TAG...${NC}"

# Check if we're on the main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "master" ] && [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${YELLOW}Warning: You're not on master/main branch. Current branch: $CURRENT_BRANCH${NC}"
    echo "Do you want to continue? (y/n)"
    read CONTINUE
    if [ "$CONTINUE" != "y" ]; then
        exit 1
    fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: You have uncommitted changes. Please commit or stash them first.${NC}"
    exit 1
fi

# Pull latest changes
echo "Pulling latest changes..."
git pull origin $CURRENT_BRANCH

# Run tests
echo -e "${GREEN}Running tests...${NC}"
python -m pytest tests/ -v --tb=short || {
    echo -e "${RED}Tests failed. Please fix them before releasing.${NC}"
    exit 1
}

# Build the package to verify it works
echo -e "${GREEN}Building package...${NC}"
pip install build
python -m build

# Create tag
echo -e "${GREEN}Creating tag $TAG...${NC}"
git tag -a "$TAG" -m "Release $TAG

- Added 45 new language modules
- Total 119+ language codes supported
- Full implementations with native language support
- Comprehensive test coverage
"

# Push tag
echo -e "${GREEN}Pushing tag to remote...${NC}"
git push origin "$TAG"

echo -e "${GREEN}✅ Release $TAG created successfully!${NC}"
echo ""
echo "GitHub Actions will now:"
echo "1. Build the distribution packages"
echo "2. Run tests on all Python versions"
echo "3. Create a GitHub release with artifacts"
echo "4. Publish to PyPI (if configured)"
echo ""
echo "Monitor progress at: https://github.com/gladiaio/num2words2/actions"
