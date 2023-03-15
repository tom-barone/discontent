# 💚 Discontent

<div>
<a href="https://chrome.google.com/webstore/detail/discontent/kglbdhongcfkafgfgofpgaehafnbgnhd" target="_blank"><img height=70 src="docs/assets/button_chrome.png" alt="Get Discontent for Chrome"></img></a>
<a href="https://addons.mozilla.org/addon/discontent" target="_blank"><img height=70 src="docs/assets/button_firefox.png" alt="Get Discontent for Firefox"></img></a>
</div>

[![Chrome Web Store](https://img.shields.io/chrome-web-store/users/kglbdhongcfkafgfgofpgaehafnbgnhd?label=Chrome%20users&color=blue)](https://chrome.google.com/webstore/detail/discontent/kglbdhongcfkafgfgofpgaehafnbgnhd)
[![Mozilla Add-on](https://img.shields.io/amo/users/discontent?label=Firefox%20users&color=blue)](https://addons.mozilla.org/addon/discontent)
[![Chrome Web Store](https://img.shields.io/chrome-web-store/stars/kglbdhongcfkafgfgofpgaehafnbgnhd?label=Chrome%20rating)](https://chrome.google.com/webstore/detail/discontent/kglbdhongcfkafgfgofpgaehafnbgnhd)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/tom-barone/Discontent/continuous-integration.yml?label=Build)](https://github.com/tom-barone/Discontent/actions/workflows/continuous-integration.yml)

This aims to be an open, crowdsourced browser extension to fight garbage content on the web.

As of writing, if I type `difference between reddit and twitter` into [Google](https://www.google.com/search?q=difference%20between%20reddit%20and%20twitter), in the second result I get [this](https://askanydifference.com/difference-between-reddit-and-twitte/):

> Reddit is dependent on the users of the website. They need to not be registered users. Whereas, Twitter is dependent on the general public who are registered profile users of Twitter along with their editors.

Clearly this website is complete trash. With the advent of incredible AI tools like ChatGPT, I fear everyone out there that makes interesting and quality content will soon be buried by an immense ocean of AI generated, SEO optimised nonsense. All you lovely people producing good stuff, all I want to do is _find_ you. Use this extension to do so.

Inspired by my **deep hate** of those AI customer support bots. And immoral SEO consultants.

## How it works?

It's basically a like / dislike system, but for websites.

<img height=200 src="docs/assets/screenshot_good_and_bad_links_cropped.jpg" alt="Links with icons prepended"></img>

- If a website is in the database and it has a bad score, it'll prepend all your links with a ❌.
- If it's spicy and there's lots of votes both ways, you see a 🤨.
- If it's awesome and deserves to be cherished, you see a 💚.
- When there aren't enough ratings, nothing will show up.

💚 When you find a site that is a beautiful smiling breath of fresh air; use the extension popup to vote and share your love.

❌ When you find a site that kicks down your door, calls you stupid and holds out its greasy hand demanding ad revenue; use the popup to warn the next bloke.

Works for Google, Bing & DuckDuckGo.

## Motivation

Historically we relied on the big search engines to filter out the good from the bad, and generally speaking they did a pretty good job. But all this generative AI stuff has them spooked and now we have to wade through a lot of rubbish to find anything half decent.

As an aside, rating platforms like this become useless when they start accepting money in exchange for ratings. My promise is that Discontent will never do this, so don't bother asking.

## Initial Data

To get things off the ground, I've used the [front page submissions list](https://news.ycombinator.com/lists) from [HackerNews](https://news.ycombinator.com/news). I've gone back a year and taken the best 30 from each day to build a set of good links.

There is also some excellent work done by the legends at [uBlacklist](https://iorate.github.io/ublacklist/docs) [[1](https://github.com/arosh/ublacklist-github-translation),[2](https://github.com/arosh/ublacklist-stackoverflow-translation),[3](https://github.com/franga2000/aliexpress-fake-sites)], [Content Farm List](https://github.com/wdmpa/content-farm-list) & [Content Farm Terminator](https://danny0838.github.io/content-farm-terminator/en/). Using those lists I've compiled a set of initial bad links as well.

## Configuration

You can change the good / spicy / bad icons in the settings to something custom. Perhaps something like this?

| Setting | Icon |
| ------- | :--: |
| Good    |  😍  |
| Spicy   |  🌶   |
| Bad     |  🤮  |

## Technical

[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=tom-barone_Discontent&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=tom-barone_Discontent)
[![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=tom-barone_Discontent&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=tom-barone_Discontent)
[![Maintainability Rating](https://sonarcloud.io/api/project_badges/measure?project=tom-barone_Discontent&metric=sqale_rating)](https://sonarcloud.io/summary/new_code?id=tom-barone_Discontent)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=tom-barone_Discontent&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=tom-barone_Discontent)

When you fire up a search engine, it'll grab a list of all the relevant links on the page, then hit an API asking for their scores.
When a user submits a vote it stores the vote, the timestamp and a randomly generated UUID for that user. No other user data is stored.

The extension itself is written in Typescript. The web API runs off an AWS lambda built in Rust, and the whole thing is backed by DynamoDB. There's some piecemeal python & ruby scripts for testing.

For the nuts and bolts, see the [Architecture](./docs/architecture.md) page.

For instructions on building, testing & developing see the [Contributing](./docs/contributing.md) page.

## Privacy

It's an important thing. Take a look at the [Privacy](./docs/privacy.md) page.
