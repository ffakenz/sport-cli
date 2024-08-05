# ⚽ sport-cli

**sport-cli** is a powerful command-line interface (CLI) tool designed for querying and analyzing data from sport events. 

It interfaces with sport data providers like [SportRadar](https://sportradar.com/) to deliver up-to-date **football** information with ease.

It comes with several powerful features to enhance your experience

- 🚀 **Advanced Analytics**: Dive deep into sports data with detailed metrics such as top scorers, assists, and more.
- 🔍 **Comprehensive Search**: Easily find competitions, seasons, teams, and players.
- 🧩 **Flexible Queries**: Customize your searches with various metrics, dimensions, and sorting options to fit your needs.
- 🆕 **Latest Data**: Always up-to-date. Stay current with the most recent data from supported sports providers.

## 🔧 Installation

> **Note:** Ensure you have Rust and Cargo installed on your system. For installation instructions, visit [the Rust installation guide](https://www.rust-lang.org/tools/install).

To install `sport-cli`, use Cargo, the Rust package manager. 

```sh
cargo install --path ./app --bin sport-cli
```

## ⚙️ Configuration

Before using `sport-cli`, you need to obtain an API key from the supported data provider. Follow these steps to set up the [SportRadar Soccer API](https://developer.sportradar.com/soccer/reference/soccer-api-overview):

1. **Set up Sportradar Soccer API**:
   - **Log In**: Create an account, if you haven't already, and log in to [Sportradar Console](https://console.sportradar.com/signup).
   - **Complete Registration**: Follow the on-screen registration steps to set up your account.
   - **Add Trials**: After logging in, navigate to the 'Add Trials' section.
   - **Select Soccer API**: Choose 'Soccer API' from the list of available APIs.
   - **Obtain API Key**: Follow the prompts to generate your API key.

2. **Configure Your API Key**:
   - Set your API key as an environment variable on your local system. This allows `sport-cli` to access the data provider’s API.

   ```sh
   export API_KEY=your_api_key_here
   ```

## 🌈 Features

- [x] **Analytics**: Query sports data, such as top scorers or most assists players for a given competition season.

### Later:

- [ ] **Competitions**: List available sports competitions.
- [ ] **Seasons**: Retrieve details about different competition seasons.
- [ ] **Teams**: Get information on teams for a given season.
- [ ] **Players**: List players for a specific season.

## 📖 Usage

### Analytics

- **Top Scorers**
Fetch the top 10 players who scored the most in the Premier League's 23/24 season.

    ```sh
    API_KEY=$API_KEY sport-cli analytics \
        --sport football \
        --event "Premier League" \
        --location England \
        --season-start 2023-08-11 \
        --season-end 2024-05-19 \
        --dimension player \
        --metric score \
        --gender male \
        --sort desc \
        --limit 10 \
        --timeout 2000
    ```

- **Top Assist Providers**
Retrieve the top 10 players who provided the most assists in the same competition season.

    ```sh
    API_KEY=$API_KEY sport-cli analytics \
        --sport football \
        --event "Premier League" \
        --location England \
        --season-start 2023-08-11 \
        --season-end 2024-05-19 \
        --dimension player \
        --metric assist \
        --gender male \
        --sort desc \
        --limit 10 \
        --timeout 2000
    ```

## 🛠️ Troubleshooting

If you encounter any issues:

1. Ensure your API key is correctly set in the environment variable `API_KEY`.
2. Review the command syntax and options.

For further assistance, please refer to the [Documentation](#) or contact support at [franco.testagrossa@gmail.com](mailto:franco.testagrossa@gmail.com).

## 🤝 Contributing

Contributions are welcome! 

Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for details on how to contribute to this project.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
