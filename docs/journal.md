# Journal

I figured I'd take some time out to document the process of this side project, but on a more personal level. The idea came from when someone posted the first review of the extension to the Chrome store, and it made me so happy I wanted to tell the world.

Ironically though this isn't for anyone else to read, really only doing it for my own sake.

# 15-03-2023

The extension has been up and running for almost a week now and has a modest user base of about 90 people.

Someone just posted this review to the Chrome store:

> The best extension I have in my browser. It really has the potential to help me a lot with the recent upcoming struggles of just pure SEO results coming up and actual useful info getting burried below. That problem actually drives me crazy sometimes.
> I hope this gets more attention, by good ppl only ofc <3.

I don't know who you are `ohnesinn` but you are actual the biggest legend.

Some of the design choices I made for the NoSQL database were a bit misguided. Where I thought I needed GSI's to accomplish certain tasks, turns out I can just use the main table itself. Also I definitely misunderstood the usage of sort keys, and thought I could query with them without providing a PK but nope.

I've made a couple of promotional posts on r/chrome, r/firefox and HackerNews but don't really how else to promote it. Perhaps asking existing users to share it and leveraging the fact that if more people it, the more useful it'll become.

People seem preoccupied with spam prevention, though that might just the tech minded folks doing a bit of bike shedding. Apart from the simple prevention stuff already in the extension I'm not gonna give it much more thought until it becomes a problem. Interestingly it looks like someone already cracked it, a few days ago there were a bunch of votes on DuckDuckGo from these user IDs, all within the span of a couple minutes.

```
8fe16b70-f4b9-475c-90c5-da078c28381c
8fe13b70-f4b9-475c-90c5-da078c28381c
8fe11b70-f4b9-475c-90c5-da278c28381c
8fe11b70-f4b9-475c-90c5-da278c28185c
8fe11b70-f4b9-475c-90c5-da278c28185b
8fe11b70-f4b9-475c-90c5-da278c28181c
8fe11b70-f4b9-4757-98c4-da298c28285b
8fe11b70-f4b9-4757-98c4-da298c28282b
8fe11b70-f4b9-4757-98c4-da298c28185b
8fe11b70-f4b9-4757-98c4-da278c28186b
8fe11b70-f4b9-4757-95c4-da278c28185b
8fe11b70-f4b9-4756-95c4-da278c28185b
8fe11b70-f4b9-4756-94c4-da278c28185b
8fe11b70-f4b9-4756-93c4-da278c28185b
8fe11b70-f4b9-4756-92c4-da278c28185b
8fe11b70-f4b9-4756-91c5-da278c28185b
8fe11b70-f4b9-4756-91c4-da278c28185b
8fe11b70-f4b9-4756-90c5-da278c28185b
```

I wondered if they would go on to do something more nefarious and so kept an eye out for any more shenanigans, but nope. I guess they were satisfied with themselves that they found a hack and then moved on - fine by me. Obviously it's not exactly a difficult hack to pull off, but enough to prevent people with 0 programming ability from causing mischief.
