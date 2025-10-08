#!/bin/bash
# 🦀 Rust Weather API - cURL Examples
# Run the server first: cargo run --bin server

echo "🦀 Rust Weather API - cURL Test Examples"
echo "=========================================="
echo

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SERVER="http://localhost:3000"

# Test 1: Health Check
echo -e "${BLUE}📊 Test 1: Health Check${NC}"
echo "Command: curl $SERVER/"
echo "Response:"
curl -s $SERVER/ | jq '.'
echo
echo "---"
echo

# Test 2: Get all cities
echo -e "${BLUE}🌍 Test 2: Get All Available Cities${NC}"
echo "Command: curl $SERVER/cities"
echo "Response:"
curl -s $SERVER/cities | jq '.'
echo
echo "---"
echo

# Test 3: Get weather for specific cities
echo -e "${BLUE}🌤️  Test 3: Get Weather for Specific Cities${NC}"
echo 'Command: curl -X POST $SERVER/weather -H "Content-Type: application/json" -d ...'
echo "Response:"
curl -s -X POST $SERVER/weather \
  -H "Content-Type: application/json" \
  -d '{"cities": ["Stockholm", "Gaza", "Paris", "Dubai", "Tokyo"]}' \
  | jq '.'
echo
echo "---"
echo

# Test 4: Get weather statistics (sorted by temperature)
echo -e "${BLUE}📈 Test 4: Get Weather Statistics (sorted by temperature)${NC}"
echo "Command: curl '$SERVER/stats?sort=temp'"
echo "Response (first 5 coldest cities):"
curl -s "$SERVER/stats?sort=temp" | jq '.cities[:5]'
echo
echo "Summary:"
curl -s "$SERVER/stats?sort=temp" | jq '{total_cities, average_temp, hottest_city, coldest_city}'
echo
echo "---"
echo

# Test 5: Get weather statistics (sorted by city name)
echo -e "${BLUE}🏙️  Test 5: Get Weather Statistics (sorted by name)${NC}"
echo "Command: curl '$SERVER/stats?sort=name'"
echo "Response (first 5 cities alphabetically):"
curl -s "$SERVER/stats?sort=name" | jq '.cities[:5]'
echo
echo "---"
echo

# Test 6: Error handling - empty cities list
echo -e "${BLUE}❌ Test 6: Error Handling - Empty Cities List${NC}"
echo 'Command: curl -X POST $SERVER/weather -H "Content-Type: application/json" -d '"'"'{"cities": []}'"'"
echo "Response:"
curl -s -X POST $SERVER/weather \
  -H "Content-Type: application/json" \
  -d '{"cities": []}' \
  | jq '.'
echo
echo "---"
echo

# Test 7: Error handling - too many cities
echo -e "${BLUE}⚠️  Test 7: Error Handling - Too Many Cities (>20)${NC}"
echo "Command: Requesting 25 cities (limit is 20)"
echo "Response:"
curl -s -X POST $SERVER/weather \
  -H "Content-Type: application/json" \
  -d '{"cities": ["city1","city2","city3","city4","city5","city6","city7","city8","city9","city10","city11","city12","city13","city14","city15","city16","city17","city18","city19","city20","city21","city22","city23","city24","city25"]}' \
  | jq '.'
echo
echo "---"
echo

# Test 8: Get weather for unknown city
echo -e "${BLUE}🔍 Test 8: Get Weather for Unknown City (defaults to 20°C)${NC}"
echo 'Command: curl -X POST $SERVER/weather -H "Content-Type: application/json" -d '"'"'{"cities": ["UnknownCity"]}'"'"
echo "Response:"
curl -s -X POST $SERVER/weather \
  -H "Content-Type: application/json" \
  -d '{"cities": ["UnknownCity"]}' \
  | jq '.'
echo
echo "---"
echo

echo -e "${GREEN}✅ All cURL tests completed!${NC}"
echo
echo "💡 Tips:"
echo "   - Add 'jq' for pretty JSON output: brew install jq"
echo "   - Change sort parameter: ?sort=temp, ?sort=name, ?sort=humidity, ?sort=wind"
echo "   - Maximum 20 cities per request"
echo
