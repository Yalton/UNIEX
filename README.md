<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/Yalton/UNIEX">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Universal Extractor</h3>

  <p align="center">
    project_description
    <br />
    <a href="https://github.com/Yalton/UNIEX"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/Yalton/UNIEX">View Demo</a>
    ·
    <a href="https://github.com/Yalton/UNIEX/issues">Report Bug</a>
    ·
    <a href="https://github.com/Yalton/UNIEX/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

This repository contains a dynamic web scraping tool that utilizes the GPT-4 Vision API to scrape any given website and return the scraped data in Markdown format. It's designed to convert visual web content into structured Markdown text, making it easier to document, analyze, and repurpose content found across the web.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Next][Next.js]][Next-url]
* [![React][React.js]][React-url]
* [![Vue][Vue.js]][Vue-url]
* [![Angular][Angular.io]][Angular-url]
* [![Svelte][Svelte.dev]][Svelte-url]
* [![Laravel][Laravel.com]][Laravel-url]
* [![Bootstrap][Bootstrap.com]][Bootstrap-url]
* [![JQuery][JQuery.com]][JQuery-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

- Ensure you have Rust installed on your system.
- Obtain an API key from OpenAI for GPT-4 Vision API access.

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/gpt4-vision-scraper-md.git
   ```
2. Navigate to the project directory:
   ```sh
   cd UNIEX
   ```
3. Install the required dependencies:
   ```sh
   cargo update
   ```

### Usage

1. Set up your API key in the config.toml file:
   ```sh
   export OPENAI_API_KEY='your_api_key_here'
   ```
2. Use the provided scripts to start scraping:
   ```sh
   cargo run
   ```
3. Curl backend
   ```sh
    curl -X POST -H "Content-Type: application/json" -d '{"addr": "https://www.reddit.com/r/AskCulinary/comments/173g73f/is_lamb_a_gamey_meat/" http://0.0.0.0:8000/scrape_url
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

- [ ] Add frontend
- [ ] Add Database logging
- [ ] Feature 3
    - [ ] Nested Feature

See the [open issues](https://github.com/Yalton/UNIEX/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Your Name - [@yalt7117](https://twitter.com/yalt7117) - email@email_client.com

Project Link: [https://github.com/Yalton/UNIEX](https://github.com/Yalton/UNIEX)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* []()
* []()
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/Yalton/UNIEX.svg?style=for-the-badge
[contributors-url]: https://github.com/Yalton/UNIEX/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Yalton/UNIEX.svg?style=for-the-badge
[forks-url]: https://github.com/Yalton/UNIEX/network/members
[stars-shield]: https://img.shields.io/github/stars/Yalton/UNIEX.svg?style=for-the-badge
[stars-url]: https://github.com/Yalton/UNIEX/stargazers
[issues-shield]: https://img.shields.io/github/issues/Yalton/UNIEX.svg?style=for-the-badge
[issues-url]: https://github.com/Yalton/UNIEX/issues
[license-shield]: https://img.shields.io/github/license/Yalton/UNIEX.svg?style=for-the-badge
[license-url]: https://github.com/Yalton/UNIEX/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 