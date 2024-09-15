import os
import re
import random
from datetime import datetime, timedelta

# Get today's date
today = datetime.today()

# Calculate the date two months ago from today
two_months_ago = today - timedelta(days=60)

# Generate a list of all dates between two_months_ago and today
all_dates = [two_months_ago + timedelta(days=i) for i in range((today - two_months_ago).days + 1)]

# Randomly choose 20 days from this list
random_days = random.sample(all_dates, 20)

# Format the randomly chosen days into 'YYYY-MM-DD'
dates = [day.strftime('%Y-%m-%d') for day in random_days]

# Regular expression to match dates in yyyy-mm-dd format
date_pattern = re.compile(r'\d{4}-\d{2}-\d{2}')

# Walk through the current working directory recursively
for root, dirs, files in os.walk(os.path.join(os.getcwd(), 'content/post')):
    for file in files:
        if file.endswith('.md'):
            file_path = os.path.join(root, file)
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()

                # Search for all dates in the file content
                matches = date_pattern.findall(content)
                if matches:
                    # Replace each found date with a random one from the list
                    for match in matches:
                        new_date = random.choice(dates)
                        print(f"Replacing date {match} with {new_date} in {file_path}")
                        content = content.replace(match, new_date, 1)  # Replace the date only once

            # Write the updated content back to the file
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
