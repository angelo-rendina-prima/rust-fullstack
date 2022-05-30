FROM rusttodo
EXPOSE 8080
COPY . /app
WORKDIR /app
CMD ["cargo", "make", "build", "&&", "cargo", "make", "serve"]
