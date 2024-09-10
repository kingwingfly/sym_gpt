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



<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">sym_gpt</h3>

  <p align="center">
    `sym_gpt` use chatGPT to convert latex to python's `sympy` code and solve it.
    <br />
    <a href="https://docs.rs/encrypt_config"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/kingwingfly/sym_gpt">View Demo</a>
    ·
    <a href="https://github.com/kingwingfly/sym_gpt/issues">Report Bug</a>
    ·
    <a href="https://github.com/kingwingfly/sym_gpt/issues">Request Feature</a>
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
    <li><a href="#usage">Usage</a></li>
    <li><a href="#changelog">Changelog</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://github.com/kingwingfly/sym_gpt)

`sym_gpt` use chatGPT to convert latex to python's `sympy` code and solve it.

![taylor](images/taylor_expand.png)
![complicated](images/complicated.png)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* Rust

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage
```sh
# set up python venv
# e.g. for fish shell
python -m venv venv
source venv/bin/activate.fish

# install python dependencies
pip install sympy mpmath

# run
./sym

deactivate
```
If you're using `uv`, just `uv sync`.

_For more examples, please refer to the [Example](https://github.com/kingwingfly/sym_gpt/tree/dev/tests) or [Documentation](https://docs.rs/encrypt_config)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CHANGELOG -->
## Changelog

[more detailed changelog](https://github.com/kingwingfly/sym_gpt/blob/dev/CHANGELOG.md)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

...

See the [open issues](https://github.com/kingwingfly/sym_gpt/issues) for a full list of proposed features (and known issues).

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

Louis - 836250617@qq.com

Project Link: [https://github.com/kingwingfly/sym_gpt](https://github.com/kingwingfly/sym_gpt)

<p align="right">(<a href="#readme-top">back to top</a>)</p>




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/kingwingfly/sym_gpt.svg?style=for-the-badge
[contributors-url]: https://github.com/kingwingfly/sym_gpt/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/kingwingfly/sym_gpt.svg?style=for-the-badge
[forks-url]: https://github.com/kingwingfly/sym_gpt/network/members
[stars-shield]: https://img.shields.io/github/stars/kingwingfly/sym_gpt.svg?style=for-the-badge
[stars-url]: https://github.com/kingwingfly/sym_gpt/stargazers
[issues-shield]: https://img.shields.io/github/issues/kingwingfly/sym_gpt.svg?style=for-the-badge
[issues-url]: https://github.com/kingwingfly/sym_gpt/issues
[license-shield]: https://img.shields.io/github/license/kingwingfly/sym_gpt.svg?style=for-the-badge
[license-url]: https://github.com/kingwingfly/sym_gpt/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[product-screenshot]: images/screenshot.png
