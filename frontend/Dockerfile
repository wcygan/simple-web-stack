# Use an official Deno image
FROM denoland/deno:latest

# The port that your application listens to.
EXPOSE 8000

WORKDIR /app

# Copy project files first while still root
COPY . .

# Change ownership to deno user so deno.lock can be written
# Switch to root to chown, then back to deno
USER root
RUN chown -R deno:deno /app

# Prefer not to run as root.
USER deno

# Cache dependencies
RUN deno cache main.ts

# Run the application
CMD ["run", "--allow-net", "--allow-read", "--allow-env", "main.ts"] 