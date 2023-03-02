import unittest
from hacker_news_scraper import date_range, get_submissions
from datetime import datetime
from bs4 import BeautifulSoup


class TestScraper(unittest.TestCase):

    def test_date_range(self):
        start = datetime(2022, 12, 29)
        end = datetime(2023, 1, 4)

        dates = list(date_range(start, end))
        self.assertEqual(dates[0], datetime(2022, 12, 29))
        self.assertEqual(dates[1], datetime(2022, 12, 30))
        self.assertEqual(dates[2], datetime(2022, 12, 31))
        self.assertEqual(dates[3], datetime(2023, 1, 1))
        self.assertEqual(dates[4], datetime(2023, 1, 2))
        self.assertEqual(dates[5], datetime(2023, 1, 3))
        self.assertEqual(dates[6], datetime(2023, 1, 4))
        self.assertEqual(len(dates), 7)

    def test_scraping(self):
        # Truth set for the html file in test/hacker_news_front_2023_01_11.html
        truth_set = [
            ('https://news.ycombinator.com/item?id=34338995', 123),
            ('https://elkue.com/nyc-slice/', 670),
            ('https://github.com/karpathy/nanoGPT', 1532),
            ('https://news.ycombinator.com/item?id=34322303', 461),
            ('https://www.beautifulpublicdata.com/the-style-guide-for-americas-highways-mutcd/',
             198),
            ('https://www.starfivetech.com/en/site/new_details/976', 144),
            ('https://github.com/DesktopECHO/T95-H616-Malware', 221),
            ('https://www.val.town/', 332),
            ('https://devblogs.microsoft.com/oldnewthing/20230109-00/?p=107685',
             230),
            ('https://github.com/ToolJet/ToolJet/releases/tag/v2.0.0', 210),
            ('https://github.com/sourcegraph/conc', 254),
            ('https://lcamtuf.coredump.cx/gcnc/', 383),
            ('https://lateblt.tripod.com/bit68.txt', 452),
            ('https://www.sapiens.org/culture/lebanon-solar-power/', 214),
            ('https://github.com/toblotron/praxis-ide', 167),
            ('https://renato.athaydes.com/posts/unison-revolution.html', 296),
            ('https://www.allaboutcircuits.com/textbook/', 315),
            ('https://findthatmeme.com/blog/2023/01/08/image-stacks-and-iphone-racks-building-an-internet-scale-meme-search-engine-Qzrz7V6T.html',
             785),
            ('https://store.steampowered.com/app/1261430/Kandria/', 487),
            ('https://www.infoq.com/news/2022/12/apple-swift-foundation-rewrite/',
             434),
            ('https://www.vqronline.org/essay/john-hughes-goes-deep-unexpected-heaviosity-ferris-bueller%E2%80%99s-day',
             230),
            ('http://uu.diva-portal.org/smash/record.jsf?pid=diva2%3A1721987&dswid=-4818',
             165), ('https://usbc.wtf/', 214),
            ('https://networkx.org/documentation/stable/release/release_3.0.html',
             195),
            ('https://www.nytimes.com/2023/01/09/climate/ozone-hole-restoration-montreal-protocol.html',
             152), ('https://htmlwithsuperpowers.netlify.app/', 155),
            ('https://en.wikipedia.org/wiki/Grandma_Gatewood', 121),
            ('https://replicate.com/andreasjansson/cantable-diffuguesion', 68),
            ('https://arxiv.org/abs/2301.03149', 139),
            ('https://zapier.com/blog/secondments-at-zapier/', 91)
        ]

        with open('fixtures/scraper/hacker_news_front_2023_01_11.html',
                  'r') as f:
            html = f.read()
            soup = BeautifulSoup(html, 'html.parser')

            submissions = get_submissions(soup)
            self.assertEqual(len(submissions), 30)
            self.assertEqual(submissions, truth_set)


if __name__ == '__main__':
    unittest.main()
