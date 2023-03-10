<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">ex_lib</h3>

  <p align="center">
    A Rust library for various Crypto exchanges (in-dev)
    <br />
</div>

<!-- ABOUT THE PROJECT -->
## About

ex_lib is a Rust SDK for various crypto exchange APIs.
Most (if not all) exchanges offer robust APIs with many features
for programmatic trading. However, very few offer equally robust SDKs.
Imagine if Amazon didn't make libraries for AWS; wouldn't that be a pain?
This project aims to create an off-the-shelf solution to this.

Also, I wanted to learn Rust and thought this would be an interesting project :)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- GETTING STARTED -->
## Getting Started

ex_lib is currently is still in early development.
When the first exchange supported has been sufficiently fleshed-out and refined,
it will be made available on Cargo. For now, please feel free to contribute!

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

This is a project I am working on in my free time so no timeframe
will be given for the roadmap. 

If you've looked at the code, you'll see various todos scattered around.
Here are the short-term goals for this project:

- [ ] Implement return structs for API responses using serde
- [ ] Robust error-handling (so far it's just been happy-path to try and flesh-out the library)
- [ ] Comprehensive type system with Enums covering exhaustive & pre-defined parameters (e.g. tickers <ETH, BTC>, symbols <SPOT_ETH_USDT>, etc...)
- [ ] Finish first exchange implementation (currently Woo Exchange)

Please note that the first exchange implementation may change as Woo have some inconsistencies in their API and docs so I'm trying to reach out
to them.

Longer-term goals include adding more exchanges and potentially adding on-chain DEXs such as GMX, GNS, DYDX...

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- LICENSE -->
## License

Coming soon to a license.txt near you.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

Placeholder for now :)

* []()
* []()
* []()

<p align="right">(<a href="#readme-top">back to top</a>)</p>
