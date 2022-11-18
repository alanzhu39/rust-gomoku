<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">Rust Gomoku</h3>
  <p align="center">
    Backend service for PlayGomoku.net
  </p>
</div>


<!-- ABOUT THE PROJECT -->
## About The Project

Rust Gomoku provides the backend APIs for [PlayGomoku.net](https://www.playgomoku.net). Features implemented:
- Websocket server: manages client connections and maintains socket connections across user sessions using session tokens
- Lobby state: allows users to create and join lobbies, keeping server-side lobby state data
- Game state: keeps track of game state and implements game logic to respond to player actions
- Asynchronous architecture: uses the asynchronous actors model to handle asynchronous messages from different users, and to parallelize actions across separate lobbies and games

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

[![Rust][rust-lang]][rust-url]
[![Actix Web][actix-web]][actix-web-url]

### Deployed with

Build process with Terraform at the repository [gomoku-terraform](https://github.com/alanzhu39/gomoku-terraform).

[![Docker][docker]][docker-url]
[![Terraform][terraform]][terraform-url]
[![Azure][Azure]][Azure-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Rust Style Guide](https://github.com/rust-lang/fmt-rfcs)
* [Markdown Badges](https://ileriayo.github.io/markdown-badges/)
* [Actor Model in a Nutshell](https://medium.com/@KtheAgent/actor-model-in-nutshell-d13c0f81c8c7)
* [Best README Template](https://github.com/othneildrew/Best-README-Template)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/alanzhu39/rust-gomoku.svg?style=for-the-badge
[contributors-url]: https://github.com/alanzhu39/rust-gomoku/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/alanzhu39/rust-gomoku.svg?style=for-the-badge
[forks-url]: https://github.com/alanzhu39/rust-gomoku/network/members
[stars-shield]: https://img.shields.io/github/stars/alanzhu39/rust-gomoku.svg?style=for-the-badge
[stars-url]: https://github.com/alanzhu39/rust-gomoku/stargazers
[issues-shield]: https://img.shields.io/github/issues/alanzhu39/rust-gomoku.svg?style=for-the-badge
[issues-url]: https://github.com/alanzhu39/rust-gomoku/issues
[license-shield]: https://img.shields.io/github/license/alanzhu39/rust-gomoku.svg?style=for-the-badge
[license-url]: https://github.com/alanzhu39/rust-gomoku/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/feed/
[rust-lang]: https://img.shields.io/badge/rust-B7410E.svg?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[actix-web]: https://img.shields.io/badge/Actix%20Web-%23000000.svg?style=for-the-badge
[actix-web-url]: https://actix.rs/
[docker]: https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white
[docker-url]: https://www.docker.com/
[terraform]: https://img.shields.io/badge/terraform-%235835CC.svg?style=for-the-badge&logo=terraform&logoColor=white
[terraform-url]: https://www.terraform.io/
[azure]: https://img.shields.io/badge/azure-%230072C6.svg?style=for-the-badge&logo=microsoftazure&logoColor=white
[azure-url]: https://azure.microsoft.com/en-us/