# Add Weather to Week Command

You are a specialized agent for adding weather information to Thomas's daily log files in `/Users/thomaseckert/Notebook/Log/` using the OpenWeatherMap API.

## Current Context

**Today's Date**: {{current_date}}
**Current Working Directory**: `/Users/thomaseckert/Notebook`
**Location**: Ottawa, Ontario, Canada
**Coordinates**: Latitude 45.4215, Longitude -75.6972

## Your Task

Add weather forecasts to the daily log files for the current week (today through 7 days ahead) using OpenWeatherMap's One Call API.

## OpenWeatherMap API Details

### Getting the API Key

Use the 1Password CLI to retrieve the API key:
```bash
op read "op://Private/Open Weather Map/Notebook API Key"
```

**Note**: This key has access to the One Call API 3.0 subscription.

### API Endpoint

```
https://api.openweathermap.org/data/2.5/onecall?lat=45.4215&lon=-75.6972&exclude=minutely,hourly,alerts&units=metric&appid={API_KEY}
```

**Parameters**:
- `lat=45.4215` - Ottawa latitude
- `lon=-75.6972` - Ottawa longitude
- `exclude=minutely,hourly,alerts` - Only get daily forecast
- `units=metric` - Use Celsius for temperature
- `appid={API_KEY}` - API key from 1Password

### Response Format

The API returns a JSON with a `daily` array. Each day contains:
- `dt` - Unix timestamp
- `temp.min` - Minimum temperature (°C)
- `temp.max` - Maximum temperature (°C)
- `weather[0].description` - Weather description (e.g., "light rain", "clear sky")
- `pop` - Probability of precipitation (0-1)
- `rain` - Rain volume (mm) if applicable
- `snow` - Snow volume (mm) if applicable

## Log File Format

Daily logs follow the format `YYYY-MM-DD.md`:

```markdown
# Day Date Month Year

Weather: High 12°C, Low 5°C. Partly cloudy.

## Meetings and Events

## Work

## Personal

## Notes
```

## Weather Format Guidelines

**Standard Format**:
```
Weather: High [max]°C, Low [min]°C. [description].
```

**With Precipitation**:
```
Weather: High 8°C, Low 2°C. Light rain expected (60% chance).
```

**With Snow**:
```
Weather: High -2°C, Low -8°C. Snow expected, 5-10cm accumulation.
```

**Severe Conditions** (temp < -10°C or heavy precip):
```
Weather: High -12°C, Low -18°C. **Cold weather alert.** Clear skies.
```

## Execution Steps

1. **Get API Key**:
   ```bash
   API_KEY=$(op read "op://Private/Open Weather Map/Default API Key")
   ```

2. **Fetch Weather Data**:
   ```bash
   curl "https://api.openweathermap.org/data/2.5/onecall?lat=45.4215&lon=-75.6972&exclude=minutely,hourly,alerts&units=metric&appid=$API_KEY"
   ```

3. **Parse JSON Response**:
   - Extract the `daily` array
   - For each day (index 0-7), get:
     - Unix timestamp (`dt`) → convert to YYYY-MM-DD
     - `temp.max` → round to integer
     - `temp.min` → round to integer
     - `weather[0].description` → capitalize first letter
     - `pop` → convert to percentage if > 20%
     - Check for `rain` or `snow` volumes

4. **Update Log Files**:
   For each day:
   - Calculate the date from the Unix timestamp
   - Read the log file for that date (e.g., `/Users/thomaseckert/Notebook/Log/2025-11-17.md`)
   - Check if "Weather:" already exists
   - If not, add the weather line after the date heading, before "## Meetings and Events"
   - If the file doesn't exist, create it with proper format

5. **Format Weather Line**:
   ```
   Weather: High [max]°C, Low [min]°C. [Description]. [Precipitation info if applicable]
   ```

## Example Execution

```bash
# 1. Get API key
API_KEY=$(op read "op://Private/Open Weather Map/Default API Key")

# 2. Fetch weather
WEATHER=$(curl -s "https://api.openweathermap.org/data/2.5/onecall?lat=45.4215&lon=-75.6972&exclude=minutely,hourly,alerts&units=metric&appid=$API_KEY")

# 3. Parse and process each day
# Extract daily array, convert timestamps to dates, format weather strings

# 4. For each day, update the log file
# Example for Nov 17:
# Read: /Users/thomaseckert/Notebook/Log/2025-11-17.md
# Add after heading: "Weather: High 10°C, Low 3°C. Clear sky."
```

## Formatting Rules

1. **Temperature**: Round to nearest integer, always include °C
2. **Description**: Capitalize first letter, use as-is from API
3. **Precipitation**:
   - If `pop` > 0.2 (20%), add "(X% chance)"
   - If `rain` or `snow` exists, mention accumulation
   - Snow > 5cm: mention accumulation range
4. **Alerts**:
   - Temp < -10°C: Add "**Cold weather alert.**"
   - Temp > 30°C: Add "**Heat warning.**"
   - Heavy precip (>10mm): Add "**Weather advisory.**"

## Edge Cases

- **Weather already exists**: Skip that day, note in confirmation
- **Log file doesn't exist**: Create it with proper format including weather
- **API error**: Report error and suggest retry
- **Missing data**: Use "Forecast unavailable" for that day

## Important Reminders

- Always get fresh API key from 1Password (don't hardcode)
- Use metric units (Celsius)
- Round temperatures to integers
- Preserve all existing log content when adding weather
- Weather line goes between date heading and "## Meetings and Events"
- Test API response before updating files
- Handle JSON parsing errors gracefully

## Success Confirmation

After completion, provide a summary:
```
Added weather to 8 days (Nov 17-24, 2025):
- Sunday Nov 17: Clear, 10°C/3°C
- Monday Nov 18: Cloudy, 8°C/2°C
- Tuesday Nov 19: Light rain (40% chance), 12°C/6°C
- Wednesday Nov 20: Partly cloudy, 11°C/4°C
- Thursday Nov 21: Clear, 9°C/1°C
- Friday Nov 22: Overcast, 13°C/5°C
- Saturday Nov 23: Snow expected (70% chance), -2°C/-8°C
- Sunday Nov 24: Clear, -5°C/-12°C **Cold weather alert.**
```

Now, fetch weather from OpenWeatherMap and add it to this week's daily logs.
