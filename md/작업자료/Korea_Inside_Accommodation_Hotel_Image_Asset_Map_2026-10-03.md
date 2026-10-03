# Korea Inside — Accommodation Hotel Image Asset Map — 2026-10-03

- Status: LOCAL REORGANIZATION COMPLETE / STATIC QA PASS / NO MAIN / NO PRODUCTION
- Authority: current user-approved Accommodation Hotel Image Asset Reorganization instruction.
- Working branch: `th-localization-2026-10-03`
- Starting HEAD: `2904bd0dbc4d3db91885509aed4fbc21356ee4e2`
- Protected origin/main baseline: `43231bbc42454fbfb298eb795b18acbb3ab09426`
- Scope: image file location + runtime image path only; public wording is unchanged.
- Snapshot: 91 directory files = 89 image assets + 2 protected non-image template files.
- 15 HOTEL_CONFIRMED files were moved into 9 property folders; remaining 74 image assets were not moved.
- Source paths, SHA-256, dimensions, file sizes and reference counts below were captured before moving.
- Current paths retain exact filenames and original binary bytes. No copy, resize, conversion, recompression or deduplication.

## Inventory classification

| Classification | Image count | Action |
| --- | --- | --- |
| HOTEL_CONFIRMED | 15 | CONFIRMED_MOVED |
| SHARED | 2 | SHARED_NOT_MOVED |
| AREA | 42 | AREA_NOT_MOVED |
| INFOGRAPHIC | 30 | INFOGRAPHIC_NOT_MOVED |
| MAP | 0 | MAP_NOT_MOVED |
| UI / ICON | 0 | UI_ICON_NOT_MOVED |
| UNRESOLVED | 0 | UNRESOLVED_NOT_MOVED |

SHARED denotes common UI or actual multi-property imagery; reuse by different language siblings alone does not make a named property photograph SHARED. AREA and INFOGRAPHIC retain their semantic classification even when used on multiple pages.

## Hotel folder register

| Canonical Hotel Name | Hotel Slug | Photo files | Public display alias / note |
| --- | --- | --- | --- |
| 9 Brick Hotel | `9-brick-hotel` | 2 | Existing English property card name |
| Amanti Hotel Seoul | `amanti-hotel-seoul` | 2 | Existing English property card name |
| Holiday Inn Express Seoul Hongdae | `holiday-inn-express-seoul-hongdae` | 2 | Existing English property card name |
| JSM Studio Hongdae | `jsm-studio-hongdae` | 1 | Paradisetel building exterior; explicitly associated with lodging card |
| Junibino Hotel Hongdae | `junibino-hotel-hongdae` | 1 | Existing English property card name |
| L7 Hongdae | `l7-hongdae` | 2 | L7 HONGDAE by LOTTE HOTELS; canonical from user example and docs/hotel-database.md:38 |
| Mercure Ambassador Seoul Hongdae | `mercure-ambassador-seoul-hongdae` | 2 | Existing English property card name |
| RYSE, Autograph Collection Seoul | `ryse-autograph-collection-seoul` | 2 | RYSE, Autograph Collection; canonical from user example and docs/hotel-database.md:37,81,106 |
| Stay Here, Again | `stay-here-again` | 1 | Withus Building exterior; explicitly associated with lodging card |


Canonical labels here are asset-management identifiers and do not change public display names or recommendation copy. JSM Studio and Stay Here, Again are lodging properties; this register does not assert a full-service hotel category.

## Responsive / asset-family treatment

Every moved file has its own explicit English img/card evidence. Room and exterior photographs remain distinct images. No jpg/webp/width variant was inferred from filename patterns, and no additional unreferenced hotel-photo variant was identified. All 15 referenced files are preserved individually.

## Full image inventory

| Filename / extension | Old Path | Current Path | SHA-256 | Dimensions | Size bytes | HTML pages | Runtime refs | Classification | Status | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `9-brick-hotel-exterior.png` / .png | `images/Accommodation/9-brick-hotel-exterior.png` | `images/Accommodation/hotels/9-brick-hotel/9-brick-hotel-exterior.png` | `608cf9be693e181489c540a59f7a88ce8fcbf004432f3e48102366f2f69a45f4` | 2133 × 1145 | 5090726 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `9-brick-hotel-room.jpg` / .jpg | `images/Accommodation/9-brick-hotel-room.jpg` | `images/Accommodation/hotels/9-brick-hotel/9-brick-hotel-room.jpg` | `5af3eb38d28b54cb88871aeb42742a37acede857c56cf46b2b2a4389552fe174` | 1082 × 587 | 743113 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `accommodation-hero-v1.png` / .png | `images/Accommodation/accommodation-hero-v1.png` | `images/Accommodation/accommodation-hero-v1.png` | `8cb4dbb05cbfb83308a006759d754604f5f2865fdbe4155430b05aeaa4b88547` | 1536 × 1024 | 2446408 | 0 | 0 | SHARED | SHARED_NOT_MOVED | HIGH |
| `accommodation-hero-v1.webp` / .webp | `images/Accommodation/accommodation-hero-v1.webp` | `images/Accommodation/accommodation-hero-v1.webp` | `064890d4f0865f2af7edd03189220c966bf59aa30ad291ed23b5b706b6ecbb84` | 1536 × 1024 | 177584 | 6 | 7 | SHARED | SHARED_NOT_MOVED | HIGH |
| `amanti-hotel-seoul-exterior.jpg` / .jpg | `images/Accommodation/amanti-hotel-seoul-exterior.jpg` | `images/Accommodation/hotels/amanti-hotel-seoul/amanti-hotel-seoul-exterior.jpg` | `08da97c4e7563824ed1bdb7df3ace9bfb816595a6df7bf92a03680a7784dc350` | 2133 × 1145 | 1806007 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `amanti-hotel-seoul-room.jpg` / .jpg | `images/Accommodation/amanti-hotel-seoul-room.jpg` | `images/Accommodation/hotels/amanti-hotel-seoul/amanti-hotel-seoul-room.jpg` | `99c36d2646495bdcb7551a4dba8353608ce9bd651f9c23ac1313c76f9b9f8793` | 944 × 540 | 555985 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `couples-stay-seoul-area-guide-es.png` / .png | `images/Accommodation/couples-stay-seoul-area-guide-es.png` | `images/Accommodation/couples-stay-seoul-area-guide-es.png` | `00f057b7154abebf462ea1c708633df28816a7436f115cb960ea8b6638aab4f6` | 1536 × 1024 | 2033408 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `couples-stay-seoul-area-guide-ja.png` / .png | `images/Accommodation/couples-stay-seoul-area-guide-ja.png` | `images/Accommodation/couples-stay-seoul-area-guide-ja.png` | `421c05905e0b64ad6774923c94640aca11827ed5bcec45805226e44e3fca1c38` | 1536 × 1024 | 2046411 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `couples-stay-seoul-area-guide-th.png` / .png | `images/Accommodation/couples-stay-seoul-area-guide-th.png` | `images/Accommodation/couples-stay-seoul-area-guide-th.png` | `36cf2ea33dee9e9b8cbbb844e8feb62027688c4203d7868b6f8927802623685b` | 1536 × 1024 | 2059139 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `couples-stay-seoul-area-guide-zh-tw.png` / .png | `images/Accommodation/couples-stay-seoul-area-guide-zh-tw.png` | `images/Accommodation/couples-stay-seoul-area-guide-zh-tw.png` | `64e0fc764c8d64d8f7cb23cb012c6df5e28b22742e616a4f8ce1c166d89fc3b3` | 1536 × 1024 | 2131838 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `couples-stay-seoul-area-guide.png` / .png | `images/Accommodation/couples-stay-seoul-area-guide.png` | `images/Accommodation/couples-stay-seoul-area-guide.png` | `184d0526b2c08d4cdc1c5c4b8a09591e1486a38180fb711caae1b40464a35a9c` | 1536 × 1024 | 2126092 | 3 | 3 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `dongdaemun-ddp-night.png` / .png | `images/Accommodation/dongdaemun-ddp-night.png` | `images/Accommodation/dongdaemun-ddp-night.png` | `8ce8811e29981bd75f497864cd7f49b342e02ed790d0a2a1d310bb8bcf0186e2` | 1408 × 768 | 2555594 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `dongdaemun-ddp-night.webp` / .webp | `images/Accommodation/dongdaemun-ddp-night.webp` | `images/Accommodation/dongdaemun-ddp-night.webp` | `13687980854c2e989cfe40d01f895a25f33ba289f70ede6f00ba165b4e861b28` | 1408 × 768 | 620604 | 21 | 21 | AREA | AREA_NOT_MOVED | HIGH |
| `dongdaemun-gate-night.jpg` / .jpg | `images/Accommodation/dongdaemun-gate-night.jpg` | `images/Accommodation/dongdaemun-gate-night.jpg` | `5c58cec06bf4e47cedfc6558773a056d113fc43bd5f3eb93611277a79e626f0d` | 5472 × 3648 | 2597401 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `family-stay-seoul-area-guide-es.png` / .png | `images/Accommodation/family-stay-seoul-area-guide-es.png` | `images/Accommodation/family-stay-seoul-area-guide-es.png` | `855ee0dfb182304557667e30c975ce76922ef1aa3f11679e577790ae285f944f` | 1491 × 1055 | 1657602 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `family-stay-seoul-area-guide-ja.png` / .png | `images/Accommodation/family-stay-seoul-area-guide-ja.png` | `images/Accommodation/family-stay-seoul-area-guide-ja.png` | `73895edfdec4d3375eba18694ecb8cfd5b094b3c3ba04ae9ad3f5b500e347c15` | 1491 × 1055 | 1632557 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `family-stay-seoul-area-guide-th.png` / .png | `images/Accommodation/family-stay-seoul-area-guide-th.png` | `images/Accommodation/family-stay-seoul-area-guide-th.png` | `b369cab50c27261173dfc2352b596f175be5a4e276e17d14c662239e0c4285d2` | 1491 × 1055 | 1688233 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `family-stay-seoul-area-guide-zh-tw.png` / .png | `images/Accommodation/family-stay-seoul-area-guide-zh-tw.png` | `images/Accommodation/family-stay-seoul-area-guide-zh-tw.png` | `fe026cc8baaaad5d473694f5f98bf095a828fc3eed1463e00eab3a4fbf150ed9` | 1491 × 1055 | 1732695 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `family-stay-seoul-area-guide.png` / .png | `images/Accommodation/family-stay-seoul-area-guide.png` | `images/Accommodation/family-stay-seoul-area-guide.png` | `5b241eb23e123238e527694cf2d3e377dac5244b355d661ec180b4f2fd5442a1` | 1491 × 1055 | 1574627 | 3 | 3 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `first-time-seoul-area-guide-es.png` / .png | `images/Accommodation/first-time-seoul-area-guide-es.png` | `images/Accommodation/first-time-seoul-area-guide-es.png` | `29631b8fad10a57f0b234969562ed434b777d2c1666b4aacf16b0b3aa1345cb6` | 1536 × 1024 | 2405022 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `first-time-seoul-area-guide-ja.png` / .png | `images/Accommodation/first-time-seoul-area-guide-ja.png` | `images/Accommodation/first-time-seoul-area-guide-ja.png` | `dce63451afb87525fe26f995b475014c94699e41e0418a36b7607193621c797a` | 1536 × 1024 | 2378157 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `first-time-seoul-area-guide-th.png` / .png | `images/Accommodation/first-time-seoul-area-guide-th.png` | `images/Accommodation/first-time-seoul-area-guide-th.png` | `4b4c3d83ee2f56f5ce4fca36cdb0e26509c47fb5403979d839b119ed1d658ed9` | 1536 × 1024 | 2327911 | 1 | 2 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `first-time-seoul-area-guide-zh-tw.png` / .png | `images/Accommodation/first-time-seoul-area-guide-zh-tw.png` | `images/Accommodation/first-time-seoul-area-guide-zh-tw.png` | `cf434cc0616434c25a169990c2865d2e65bcb02333ba9e2e575d555dfa222cc2` | 1536 × 1024 | 2471566 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `first-time-seoul-area-guide.png` / .png | `images/Accommodation/first-time-seoul-area-guide.png` | `images/Accommodation/first-time-seoul-area-guide.png` | `2e958e12213927ce0a4a15588c525875ec14323eaf04486ae4e468784d988421` | 1536 × 1024 | 2653378 | 3 | 4 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `gangnam-city-2.png` / .png | `images/Accommodation/gangnam-city-2.png` | `images/Accommodation/gangnam-city-2.png` | `427751ca458da8e657b743b6ee43795c3c3a6338005e219a5f95cc90ad840504` | 1536 × 1024 | 2862288 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `gangnam-city-2.webp` / .webp | `images/Accommodation/gangnam-city-2.webp` | `images/Accommodation/gangnam-city-2.webp` | `bccd2b8f181336c7a24d7d52cc93fdea78223df55cb06ec8686ac28e36659a3d` | 1536 × 1024 | 821274 | 6 | 6 | AREA | AREA_NOT_MOVED | HIGH |
| `gangnam-city.png` / .png | `images/Accommodation/gangnam-city.png` | `images/Accommodation/gangnam-city.png` | `05dcd637a49ca15b721d2fa5730b09cc0953c9e2376953c113f461956b8fe380` | 1536 × 1024 | 2825339 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `gangnam-city.webp` / .webp | `images/Accommodation/gangnam-city.webp` | `images/Accommodation/gangnam-city.webp` | `78b6553ba0c45eee3f92012e31da48de9cc30e91fd676aa436d5b4fa5229ba03` | 1536 × 1024 | 777608 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `gangnam-station-crossroads-kto.jpg` / .jpg | `images/Accommodation/gangnam-station-crossroads-kto.jpg` | `images/Accommodation/gangnam-station-crossroads-kto.jpg` | `a1a3d931d2645aa54ec347f512b6b684da339dc68665e725a7ddfba702777215` | 5000 × 3327 | 2532483 | 6 | 6 | AREA | AREA_NOT_MOVED | HIGH |
| `gangnam-station-street-family.jpg` / .jpg | `images/Accommodation/gangnam-station-street-family.jpg` | `images/Accommodation/gangnam-station-street-family.jpg` | `a1a3d931d2645aa54ec347f512b6b684da339dc68665e725a7ddfba702777215` | 5000 × 3327 | 2532483 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `gangnam-station-street.jpg` / .jpg | `images/Accommodation/gangnam-station-street.jpg` | `images/Accommodation/gangnam-station-street.jpg` | `ee5e16eddd1cf8a80da5cb180720be939688f07e316fd343e6e3019222f3eaa9` | 5000 × 3333 | 3089270 | 28 | 28 | AREA | AREA_NOT_MOVED | HIGH |
| `holiday-inn-express-seoul-hongdae-exterior.jpg` / .jpg | `images/Accommodation/holiday-inn-express-seoul-hongdae-exterior.jpg` | `images/Accommodation/hotels/holiday-inn-express-seoul-hongdae/holiday-inn-express-seoul-hongdae-exterior.jpg` | `ab54a9e15e042c3f144c543c499a569862b2393c39adad612a03bd730b01242a` | 2133 × 1145 | 1286980 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `holiday-inn-express-seoul-hongdae-room.jpg` / .jpg | `images/Accommodation/holiday-inn-express-seoul-hongdae-room.jpg` | `images/Accommodation/hotels/holiday-inn-express-seoul-hongdae/holiday-inn-express-seoul-hongdae-room.jpg` | `ba6815318f4203bbca995871ea8db470da21471d170c761b264e2e65264aa6e0` | 684 × 483 | 379044 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `hongdae-busking.png` / .png | `images/Accommodation/hongdae-busking.png` | `images/Accommodation/hongdae-busking.png` | `a85dc458a35da91cd62ffda19fc95e5f08595682e358f41f94f5e9111725ad5e` | 1672 × 941 | 2895243 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `hongdae-busking.webp` / .webp | `images/Accommodation/hongdae-busking.webp` | `images/Accommodation/hongdae-busking.webp` | `b28ef903ce46dc08a72ed739af45f231dca01d1900d03643052906e1cd929832` | 1672 × 941 | 906106 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `hongdae-busy-main-street.webp` / .webp | `images/Accommodation/hongdae-busy-main-street.webp` | `images/Accommodation/hongdae-busy-main-street.webp` | `83105021c64bb46734231556a18fd85b705f76efbf658e820b160631a6531e49` | 5472 × 3648 | 1795874 | 42 | 42 | AREA | AREA_NOT_MOVED | HIGH |
| `hongdae-night-street.webp` / .webp | `images/Accommodation/hongdae-night-street.webp` | `images/Accommodation/hongdae-night-street.webp` | `e5bcc642bfc34f481649a9d348cb2e1a5ea3474c0d63dd2f66d8a000c3826192` | 5616 × 3744 | 866974 | 20 | 20 | AREA | AREA_NOT_MOVED | HIGH |
| `hongdae-tree-lined-street.webp` / .webp | `images/Accommodation/hongdae-tree-lined-street.webp` | `images/Accommodation/hongdae-tree-lined-street.webp` | `c93e19984eba7b842be445e4536239b42bd9d10818d8fd770e556d7f76d2314e` | 5472 × 3648 | 2484110 | 28 | 28 | AREA | AREA_NOT_MOVED | HIGH |
| `hongdae-vs-myeongdong-es.webp` / .webp | `images/Accommodation/hongdae-vs-myeongdong-es.webp` | `images/Accommodation/hongdae-vs-myeongdong-es.webp` | `76bdce275be1744df0227678ac5e9d66b5724c27726d91d8c41b43296862bd42` | 1672 × 941 | 1740064 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `hongdae-vs-myeongdong-ja.webp` / .webp | `images/Accommodation/hongdae-vs-myeongdong-ja.webp` | `images/Accommodation/hongdae-vs-myeongdong-ja.webp` | `d7b545a576b30ee2b0ccdbf09177004449f33fa2acfd80bffac099403b58c1b4` | 1672 × 941 | 1740850 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `hongdae-vs-myeongdong-th.webp` / .webp | `images/Accommodation/hongdae-vs-myeongdong-th.webp` | `images/Accommodation/hongdae-vs-myeongdong-th.webp` | `eb837dffc6b5a2d97f9047fcbdaf60d57ce50d4900eee149286a51a67ac50b8e` | 1672 × 941 | 1691794 | 1 | 2 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `hongdae-vs-myeongdong-zh-tw.webp` / .webp | `images/Accommodation/hongdae-vs-myeongdong-zh-tw.webp` | `images/Accommodation/hongdae-vs-myeongdong-zh-tw.webp` | `b596ce162548f65b6721fb0c0c86811a37edfecb618ece5010265692dbbf8601` | 1672 × 941 | 465500 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `hongdae-vs-myeongdong.webp` / .webp | `images/Accommodation/hongdae-vs-myeongdong.webp` | `images/Accommodation/hongdae-vs-myeongdong.webp` | `a1bdbff1f785e12489fc30bd3009f8c46648d5f3aba48e5dc314c4b7b6707537` | 1672 × 941 | 236616 | 3 | 4 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `insadong-shopping-street-evening.jpg` / .jpg | `images/Accommodation/insadong-shopping-street-evening.jpg` | `images/Accommodation/insadong-shopping-street-evening.jpg` | `93396c41cd72098a745679af319721e0b4bcac5408dff709b620c5953b79a406` | 9328 × 6500 | 7594241 | 34 | 34 | AREA | AREA_NOT_MOVED | HIGH |
| `insadong-traditional-masks-.jpg` / .jpg | `images/Accommodation/insadong-traditional-masks-.jpg` | `images/Accommodation/insadong-traditional-masks-.jpg` | `edea4b7159a3e32d26d4ed5a29ba03227666ada3964f1a4454b7d30ab63734a2` | 3264 × 2448 | 1971213 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `insadong-traditional-masks-.webp` / .webp | `images/Accommodation/insadong-traditional-masks-.webp` | `images/Accommodation/insadong-traditional-masks-.webp` | `da1f0acecd7451bc5705845ab7bc4e3d2d9598cde1075bd418b017af9b0342d9` | 3264 × 2448 | 2148020 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `itaewon-night-street.png` / .png | `images/Accommodation/itaewon-night-street.png` | `images/Accommodation/itaewon-night-street.png` | `474656e47e06ee92e914fcb64def41f75602f18e6470a7cb8db7f8cf312ebf72` | 1536 × 1024 | 2545238 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `itaewon-night-street.webp` / .webp | `images/Accommodation/itaewon-night-street.webp` | `images/Accommodation/itaewon-night-street.webp` | `afc720670857117ac8ef5cbd1fa1d3722bdfb7ad635e18bd028bdae24a820c49` | 1536 × 1024 | 554852 | 26 | 26 | AREA | AREA_NOT_MOVED | HIGH |
| `jamsil-lotte-world-tower-seokchon-lake.jpg` / .jpg | `images/Accommodation/jamsil-lotte-world-tower-seokchon-lake.jpg` | `images/Accommodation/jamsil-lotte-world-tower-seokchon-lake.jpg` | `99cf132c2dbb98a028fa1e61b998f1f3e96eac9ff0222ec3b03df34c10c16db9` | 5472 × 3648 | 2680426 | 13 | 13 | AREA | AREA_NOT_MOVED | HIGH |
| `jamsil-seokchon-lake-autumn.jpg` / .jpg | `images/Accommodation/jamsil-seokchon-lake-autumn.jpg` | `images/Accommodation/jamsil-seokchon-lake-autumn.jpg` | `b7cdc9c07dfbfcb914a8ca746ab5c7939910db20aa84539e79c3cbb094927640` | 4000 × 2673 | 1991519 | 21 | 21 | AREA | AREA_NOT_MOVED | HIGH |
| `jamsil-seokchon-lake.png` / .png | `images/Accommodation/jamsil-seokchon-lake.png` | `images/Accommodation/jamsil-seokchon-lake.png` | `43c8ed165e39e41c6a2d9db0299fd8d1f96452bd9485cd0013999f810ee6e45b` | 1536 × 1024 | 2875582 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `jamsil-seokchon-lake.webp` / .webp | `images/Accommodation/jamsil-seokchon-lake.webp` | `images/Accommodation/jamsil-seokchon-lake.webp` | `6a06844f9b113f403be5becad8b1343786cea705f508ecc6f174c4d5412e8166` | 1536 × 1024 | 711400 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `jsm-studio-hongdae-exterior.webp` / .webp | `images/Accommodation/jsm-studio-hongdae-exterior.webp` | `images/Accommodation/hotels/jsm-studio-hongdae/jsm-studio-hongdae-exterior.webp` | `9d73495b833f307c3c5623ac1a5424fdcb3f692791d34f7c6b28e13374a4d49b` | 2133 × 1145 | 1298010 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `junibino-hotel-hongdae-exterior.jpg` / .jpg | `images/Accommodation/junibino-hotel-hongdae-exterior.jpg` | `images/Accommodation/hotels/junibino-hotel-hongdae/junibino-hotel-hongdae-exterior.jpg` | `dbddd06d8671c6ea8b5f87c0b48f5920bc16ffcd3bdf8dfa42ee619579ba41ab` | 2133 × 1145 | 310053 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `l7-hongdae-by-lotte-hotels-exterior.webp` / .webp | `images/Accommodation/l7-hongdae-by-lotte-hotels-exterior.webp` | `images/Accommodation/hotels/l7-hongdae/l7-hongdae-by-lotte-hotels-exterior.webp` | `33c8aa2ff602900688b69ff47daedc4c42ad3c79a08cb0a1549e439e40fd8b5c` | 2133 × 1145 | 911430 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `l7-hongdae-by-lotte-hotels-room.jpg` / .jpg | `images/Accommodation/l7-hongdae-by-lotte-hotels-room.jpg` | `images/Accommodation/hotels/l7-hongdae/l7-hongdae-by-lotte-hotels-room.jpg` | `ba1d69842617cbe2b4284f20c828fce2464288088e7ee796b28f237b05699c10` | 800 × 600 | 517878 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `luxury-stay-seoul-area-guide-es.png` / .png | `images/Accommodation/luxury-stay-seoul-area-guide-es.png` | `images/Accommodation/luxury-stay-seoul-area-guide-es.png` | `2f15589c4f5ce28a48488fbe0b8e405a52e1a77c1b7e7d4d4c1a50ec489c16aa` | 1536 × 1024 | 2470290 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `luxury-stay-seoul-area-guide-ja.png` / .png | `images/Accommodation/luxury-stay-seoul-area-guide-ja.png` | `images/Accommodation/luxury-stay-seoul-area-guide-ja.png` | `af71193154b4368532595eecddc21a7ebb0020d4908961a69c6b536170a75d53` | 1536 × 1024 | 2442503 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `luxury-stay-seoul-area-guide-th.png` / .png | `images/Accommodation/luxury-stay-seoul-area-guide-th.png` | `images/Accommodation/luxury-stay-seoul-area-guide-th.png` | `f374fe690ac539e7858eca4b649f6e923f4ff6f1e67546afc66b082a13290ec5` | 1536 × 1024 | 2275732 | 0 | 0 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `luxury-stay-seoul-area-guide-zh-tw.png` / .png | `images/Accommodation/luxury-stay-seoul-area-guide-zh-tw.png` | `images/Accommodation/luxury-stay-seoul-area-guide-zh-tw.png` | `28aa8d1c3bd2751fb082c9ccb0071a4db7a591bfec5f27fd501394094ecd08a7` | 1536 × 1024 | 2608764 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `luxury-stay-seoul-area-guide.png` / .png | `images/Accommodation/luxury-stay-seoul-area-guide.png` | `images/Accommodation/luxury-stay-seoul-area-guide.png` | `a88d4eeb4a1678a82136a1bdb8893859e99544c41152c884990735e5ad364935` | 1536 × 1024 | 2715896 | 3 | 3 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `mapo-gongdeok-station.png` / .png | `images/Accommodation/mapo-gongdeok-station.png` | `images/Accommodation/mapo-gongdeok-station.png` | `f61786c44c50a3aec3f4de5a6b566ed88b47c778c3bc91fae0235abc552075cc` | 1536 × 1024 | 2792924 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `mapo-gongdeok-station.webp` / .webp | `images/Accommodation/mapo-gongdeok-station.webp` | `images/Accommodation/mapo-gongdeok-station.webp` | `762605994d47a6b08532b53b84b4d42b68c9b8a71426557e69454fc4556c714f` | 1536 × 1024 | 811116 | 20 | 20 | AREA | AREA_NOT_MOVED | HIGH |
| `mapo-pork-rib-street-day.jpg` / .jpg | `images/Accommodation/mapo-pork-rib-street-day.jpg` | `images/Accommodation/mapo-pork-rib-street-day.jpg` | `5c9df72f4aaa4b8c5ea9ac52b08e5ae7ac343983d989962fa965b1fc43310fee` | 5472 × 3648 | 3949677 | 28 | 28 | AREA | AREA_NOT_MOVED | HIGH |
| `mercure-ambassador-seoul-hongdae-exterior.jpg` / .jpg | `images/Accommodation/mercure-ambassador-seoul-hongdae-exterior.jpg` | `images/Accommodation/hotels/mercure-ambassador-seoul-hongdae/mercure-ambassador-seoul-hongdae-exterior.jpg` | `0106478d3be54596df9a4beb93cfffd9a909dfe893c506cc66dcbd8f076c44ec` | 940 × 705 | 764614 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `mercure-ambassador-seoul-hongdae-room.jpg` / .jpg | `images/Accommodation/mercure-ambassador-seoul-hongdae-room.jpg` | `images/Accommodation/hotels/mercure-ambassador-seoul-hongdae/mercure-ambassador-seoul-hongdae-room.jpg` | `ef1ee16ac99bf31c9dd646708de60372c3385103ed3f766d42014517d621d482` | 975 × 643 | 211547 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `myeongdong-crowded-shopping-street.webp` / .webp | `images/Accommodation/myeongdong-crowded-shopping-street.webp` | `images/Accommodation/myeongdong-crowded-shopping-street.webp` | `5c5d0bf25ac6bb98ea2023a436a65263528f3cdfa6d9db8c89085b32c74220d1` | 876 × 584 | 230968 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `myeongdong-day-shopping-street.webp` / .webp | `images/Accommodation/myeongdong-day-shopping-street.webp` | `images/Accommodation/myeongdong-day-shopping-street.webp` | `645ab79ff627a24246e376cb641ca109d5b2a3e77a88bd44621375b944cb316b` | 5472 × 3648 | 2076776 | 41 | 41 | AREA | AREA_NOT_MOVED | HIGH |
| `myeongdong-night-cityscape.webp` / .webp | `images/Accommodation/myeongdong-night-cityscape.webp` | `images/Accommodation/myeongdong-night-cityscape.webp` | `c62fb14f752e4d19acf03595d6c7d3dc42bf903aa4b1661d4170db36df0ec0a6` | 5000 × 2989 | 1061704 | 6 | 6 | AREA | AREA_NOT_MOVED | HIGH |
| `myeongdong-shopping-street.jpg` / .jpg | `images/Accommodation/myeongdong-shopping-street.jpg` | `images/Accommodation/myeongdong-shopping-street.jpg` | `fea72b029b68e84ffc598012b29593b163bf6cc8bf7c7087e307d65ecd842c81` | 5184 × 3456 | 2223099 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `myeongdong-shopping-street.webp` / .webp | `images/Accommodation/myeongdong-shopping-street.webp` | `images/Accommodation/myeongdong-shopping-street.webp` | `db9e55906362ec9ae7746a751ae4ccfb6129ffac20e1eb67d138874043a47574` | 5184 × 3456 | 2404020 | 14 | 14 | AREA | AREA_NOT_MOVED | HIGH |
| `nightlife-stay-seoul-area-guide-es.png` / .png | `images/Accommodation/nightlife-stay-seoul-area-guide-es.png` | `images/Accommodation/nightlife-stay-seoul-area-guide-es.png` | `854c7ff85b9087e5b2600639a18edf949c36f337aa21a5f6cc34a006737174d6` | 1536 × 1024 | 1670424 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `nightlife-stay-seoul-area-guide-ja.png` / .png | `images/Accommodation/nightlife-stay-seoul-area-guide-ja.png` | `images/Accommodation/nightlife-stay-seoul-area-guide-ja.png` | `f45776a5982cf7d3669be26518a0dc9f451471ec3155c9759fd62fa30162e33d` | 1536 × 1024 | 1602659 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `nightlife-stay-seoul-area-guide-th.png` / .png | `images/Accommodation/nightlife-stay-seoul-area-guide-th.png` | `images/Accommodation/nightlife-stay-seoul-area-guide-th.png` | `1354932d3148dccb94e007e70843ef067934b3c92524fe3c9a823d120c52b057` | 1536 × 1024 | 1594881 | 0 | 0 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `nightlife-stay-seoul-area-guide-zh-tw.png` / .png | `images/Accommodation/nightlife-stay-seoul-area-guide-zh-tw.png` | `images/Accommodation/nightlife-stay-seoul-area-guide-zh-tw.png` | `63eacd5a3484754efc75c1306c1b564855a0864938b15f1663b00cdcd2a65b3e` | 1536 × 1024 | 1637809 | 1 | 1 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `nightlife-stay-seoul-area-guide.png` / .png | `images/Accommodation/nightlife-stay-seoul-area-guide.png` | `images/Accommodation/nightlife-stay-seoul-area-guide.png` | `ae765cc088069bc9bd156b6e8f9b45e564d08ad6d4e460cfb39249401c2b5a2f` | 1536 × 1024 | 1590261 | 3 | 3 | INFOGRAPHIC | INFOGRAPHIC_NOT_MOVED | HIGH |
| `ryse-autograph-collection-exterior.webp` / .webp | `images/Accommodation/ryse-autograph-collection-exterior.webp` | `images/Accommodation/hotels/ryse-autograph-collection-seoul/ryse-autograph-collection-exterior.webp` | `844afd01c91010edf1295202a934b42921d41e55698c7f6882a16120d1f3a8b1` | 2133 × 1145 | 1063412 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `ryse-autograph-collection-room.jpg` / .jpg | `images/Accommodation/ryse-autograph-collection-room.jpg` | `images/Accommodation/hotels/ryse-autograph-collection-seoul/ryse-autograph-collection-room.jpg` | `9321b7767c7e659d4b67d16a8796b668557eaa266a58d7be475153d1227732ec` | 1024 × 803 | 1041512 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |
| `seongsu-alley.png` / .png | `images/Accommodation/seongsu-alley.png` | `images/Accommodation/seongsu-alley.png` | `54a13361dd622ca581a492b0e29c21e78bae9ce8f9523d46d2bbf598a8a014e8` | 1536 × 1024 | 3080122 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `seongsu-alley.webp` / .webp | `images/Accommodation/seongsu-alley.webp` | `images/Accommodation/seongsu-alley.webp` | `f2823d3f02c805889db10d3914c9179de9ef6a2d0c01f121fe3f24ef07673a6e` | 1536 × 1024 | 882836 | 21 | 21 | AREA | AREA_NOT_MOVED | HIGH |
| `seongsu-seoul-forest.png` / .png | `images/Accommodation/seongsu-seoul-forest.png` | `images/Accommodation/seongsu-seoul-forest.png` | `16fd5b98cf90b2a861ef0ff0d4d9575c471014383faaab3d40832caf81865f19` | 1536 × 1024 | 3488566 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `seongsu-seoul-forest.webp` / .webp | `images/Accommodation/seongsu-seoul-forest.webp` | `images/Accommodation/seongsu-seoul-forest.webp` | `b06ea59121dbfd09a7f756d0f1cf2687dd823c2017aa255b0733ac202e2c1443` | 1536 × 1024 | 1063602 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `seoul-station-city-view.jpg` / .jpg | `images/Accommodation/seoul-station-city-view.jpg` | `images/Accommodation/seoul-station-city-view.jpg` | `d607cf37011fd645e5fda41c7817dff729db15946c3cc3e539201056463b529f` | 6000 × 4000 | 3749823 | 20 | 20 | AREA | AREA_NOT_MOVED | HIGH |
| `seoul-station.png` / .png | `images/Accommodation/seoul-station.png` | `images/Accommodation/seoul-station.png` | `885fea866ecfb3801ae1b902e946173d65730a747ed88f6f9d5a31d52205c953` | 1536 × 1024 | 2972389 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `seoul-station.webp` / .webp | `images/Accommodation/seoul-station.webp` | `images/Accommodation/seoul-station.webp` | `801e43cbf54a3f1368756f26701c2dc82a6b085d739b6a61650409e6cf5453fb` | 1536 × 1024 | 538914 | 26 | 26 | AREA | AREA_NOT_MOVED | HIGH |
| `sinchon-station-night.webp` / .webp | `images/Accommodation/sinchon-station-night.webp` | `images/Accommodation/sinchon-station-night.webp` | `c58e041c6fc9450f88f02d2c4e41271299b009e7c86446d716bb767b220555f7` | 1374 × 768 | 425306 | 7 | 7 | AREA | AREA_NOT_MOVED | HIGH |
| `sinsa-garosu-gil.png` / .png | `images/Accommodation/sinsa-garosu-gil.png` | `images/Accommodation/sinsa-garosu-gil.png` | `b4c689b5e081bfc723100416602ecf5ae3f2f0cf3a7bfaed46bca2d0d63f1026` | 792 × 448 | 843012 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `sinsa-garosu-gil.webp` / .webp | `images/Accommodation/sinsa-garosu-gil.webp` | `images/Accommodation/sinsa-garosu-gil.webp` | `aca721fdc96f5133093eb02c55e52cb7eb6b8819045213d21f80013fa1a608e3` | 792 × 448 | 125674 | 0 | 0 | AREA | AREA_NOT_MOVED | HIGH |
| `stay-here-again-exterior.webp` / .webp | `images/Accommodation/stay-here-again-exterior.webp` | `images/Accommodation/hotels/stay-here-again/stay-here-again-exterior.webp` | `1413f4e39d4025e5dd997273382504179e4055bbdfd630cad0b0cda62e62a354` | 2133 × 1145 | 1070252 | 7 | 7 | HOTEL_CONFIRMED | CONFIRMED_MOVED | HIGH |

## Image-by-image attribution and reference inventory

### ASSET-001 — 9-brick-hotel-exterior.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | 9 Brick Hotel |
| Hotel Slug | 9-brick-hotel |
| Filename | `9-brick-hotel-exterior.png` |
| Extension | .png |
| Old Path | `images/Accommodation/9-brick-hotel-exterior.png` |
| New / Current Path | `images/Accommodation/hotels/9-brick-hotel/9-brick-hotel-exterior.png` |
| SHA-256 | `608cf9be693e181489c540a59f7a88ce8fcbf004432f3e48102366f2f69a45f4` |
| Dimensions | 2133 × 1145 |
| File Size | 5090726 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:503; Good-value stays → 9 Brick Hotel; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 503 | 1 |
| `es/where-to-stay-in-hongdae.html` | 503 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 503 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 503 | 1 |
| `th/where-to-stay-in-hongdae.html` | 504 | 1 |
| `where-to-stay-in-hongdae.html` | 503 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 503 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:503` | 9 Brick Hotel | Exterior of 9 Brick Hotel in Hongdae | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-002 — 9-brick-hotel-room.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | 9 Brick Hotel |
| Hotel Slug | 9-brick-hotel |
| Filename | `9-brick-hotel-room.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/9-brick-hotel-room.jpg` |
| New / Current Path | `images/Accommodation/hotels/9-brick-hotel/9-brick-hotel-room.jpg` |
| SHA-256 | `5af3eb38d28b54cb88871aeb42742a37acede857c56cf46b2b2a4389552fe174` |
| Dimensions | 1082 × 587 |
| File Size | 743113 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:504; Good-value stays → 9 Brick Hotel; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 504 | 1 |
| `es/where-to-stay-in-hongdae.html` | 504 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 504 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 504 | 1 |
| `th/where-to-stay-in-hongdae.html` | 505 | 1 |
| `where-to-stay-in-hongdae.html` | 504 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 504 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:504` | 9 Brick Hotel | Guest room at 9 Brick Hotel | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-003 — accommodation-hero-v1.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `accommodation-hero-v1.png` |
| Extension | .png |
| Old Path | `images/Accommodation/accommodation-hero-v1.png` |
| New / Current Path | `images/Accommodation/accommodation-hero-v1.png` |
| SHA-256 | `8cb4dbb05cbfb83308a006759d754604f5f2865fdbe4155430b05aeaa4b88547` |
| Dimensions | 1536 × 1024 |
| File Size | 2446408 bytes |
| Classification | SHARED |
| Hotel-specific | NO |
| Shared common UI / multi-property | YES |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | SHARED_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Both PNG/WEBP visually inspected: same text-free generic accommodation room/luggage/Seoul-view editorial scene, with no canonical named hotel. WEBP is currently used by six home-journey cards (root/en, es, ja, zh-tw, fr, de) and shared accommodation Hero CSS. PNG has no current runtime reference. Audit explicitly excludes the WEBP as a text-free hero illustration. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:571 |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-004 — accommodation-hero-v1.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `accommodation-hero-v1.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/accommodation-hero-v1.webp` |
| New / Current Path | `images/Accommodation/accommodation-hero-v1.webp` |
| SHA-256 | `064890d4f0865f2af7edd03189220c966bf59aa30ad291ed23b5b706b6ecbb84` |
| Dimensions | 1536 × 1024 |
| File Size | 177584 bytes |
| Classification | SHARED |
| Hotel-specific | NO |
| Shared common UI / multi-property | YES |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | SHARED_NOT_MOVED |
| HTML Referencing Pages | 6 |
| Runtime Reference Count | 7 |
| Attribution Basis | Both PNG/WEBP visually inspected: same text-free generic accommodation room/luggage/Seoul-view editorial scene, with no canonical named hotel. WEBP is currently used by six home-journey cards (root/en, es, ja, zh-tw, fr, de) and shared accommodation Hero CSS. PNG has no current runtime reference. Audit explicitly excludes the WEBP as a text-free hero illustration. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:571 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/index.html` | 215 | 1 |
| `es/index.html` | 215 | 1 |
| `fr/index.html` | 215 | 1 |
| `index.html` | 215 | 1 |
| `ja/index.html` | 215 | 1 |
| `style.css` | 9688 | 1 |
| `zh-tw/index.html` | 215 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `index.html:213` | A few decisions make arrival day much easier | Accommodation planning card for choosing where to stay in Korea | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | 571 | 1 |

### ASSET-005 — amanti-hotel-seoul-exterior.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Amanti Hotel Seoul |
| Hotel Slug | amanti-hotel-seoul |
| Filename | `amanti-hotel-seoul-exterior.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/amanti-hotel-seoul-exterior.jpg` |
| New / Current Path | `images/Accommodation/hotels/amanti-hotel-seoul/amanti-hotel-seoul-exterior.jpg` |
| SHA-256 | `08da97c4e7563824ed1bdb7df3ace9bfb816595a6df7bf92a03680a7784dc350` |
| Dimensions | 2133 × 1145 |
| File Size | 1806007 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:550; Away from Hongdae’s busiest streets → Amanti Hotel Seoul; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 550 | 1 |
| `es/where-to-stay-in-hongdae.html` | 550 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 550 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 550 | 1 |
| `th/where-to-stay-in-hongdae.html` | 551 | 1 |
| `where-to-stay-in-hongdae.html` | 550 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 550 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:550` | Amanti Hotel Seoul | Exterior of Amanti Hotel Seoul | Photo: Kakao Map road view |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-006 — amanti-hotel-seoul-room.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Amanti Hotel Seoul |
| Hotel Slug | amanti-hotel-seoul |
| Filename | `amanti-hotel-seoul-room.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/amanti-hotel-seoul-room.jpg` |
| New / Current Path | `images/Accommodation/hotels/amanti-hotel-seoul/amanti-hotel-seoul-room.jpg` |
| SHA-256 | `99c36d2646495bdcb7551a4dba8353608ce9bd651f9c23ac1313c76f9b9f8793` |
| Dimensions | 944 × 540 |
| File Size | 555985 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:554; Away from Hongdae’s busiest streets → Amanti Hotel Seoul; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 554 | 1 |
| `es/where-to-stay-in-hongdae.html` | 554 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 554 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 554 | 1 |
| `th/where-to-stay-in-hongdae.html` | 555 | 1 |
| `where-to-stay-in-hongdae.html` | 554 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 554 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:554` | Amanti Hotel Seoul | Guest room at Amanti Hotel Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-007 — couples-stay-seoul-area-guide-es.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `couples-stay-seoul-area-guide-es.png` |
| Extension | .png |
| Old Path | `images/Accommodation/couples-stay-seoul-area-guide-es.png` |
| New / Current Path | `images/Accommodation/couples-stay-seoul-area-guide-es.png` |
| SHA-256 | `00f057b7154abebf462ea1c708633df28816a7436f115cb960ea8b6638aab4f6` |
| Dimensions | 1536 × 1024 |
| File Size | 2033408 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:99 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `es/best-area-for-couples-seoul.html` | 263 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `es/best-area-for-couples-seoul.html:263` | Compara las zonas de Seoul para parejas de un vistazo | Guía de zonas para parejas en Seoul que compara Hongdae, Seongsu, Insadong, Myeongdong, Gangnam y Jamsil según el estilo de viaje. | Hongdae, Seongsu, Insadong, Myeongdong, Gangnam y Jamsil encajan con ritmos muy distintos, desde noches largas hasta días más pausados de cafés. |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 178, 178 | 2 |

### ASSET-008 — couples-stay-seoul-area-guide-ja.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `couples-stay-seoul-area-guide-ja.png` |
| Extension | .png |
| Old Path | `images/Accommodation/couples-stay-seoul-area-guide-ja.png` |
| New / Current Path | `images/Accommodation/couples-stay-seoul-area-guide-ja.png` |
| SHA-256 | `421c05905e0b64ad6774923c94640aca11827ed5bcec45805226e44e3fca1c38` |
| Dimensions | 1536 × 1024 |
| File Size | 2046411 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:99 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `ja/best-area-for-couples-seoul.html` | 355 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `ja/best-area-for-couples-seoul.html:355` | カップル向けソウル宿泊エリアを一覧比較 | 弘大、聖水、仁寺洞、明洞、江南、蚕室を旅行スタイル別に比較するカップル向けソウル宿泊エリアガイド | 弘大、聖水、仁寺洞、明洞、江南、蚕室は、夜遅くまで楽しむ旅から、ゆっくりカフェを巡る旅まで、合う過ごし方が大きく異なります。 |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 179, 179 | 2 |

### ASSET-009 — couples-stay-seoul-area-guide-th.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `couples-stay-seoul-area-guide-th.png` |
| Extension | .png |
| Old Path | `images/Accommodation/couples-stay-seoul-area-guide-th.png` |
| New / Current Path | `images/Accommodation/couples-stay-seoul-area-guide-th.png` |
| SHA-256 | `36cf2ea33dee9e9b8cbbb844e8feb62027688c4203d7868b6f8927802623685b` |
| Dimensions | 1536 × 1024 |
| File Size | 2059139 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:99 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `th/best-area-for-couples-seoul.html` | 360 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `th/best-area-for-couples-seoul.html:360` | เปรียบเทียบย่านในโซลสำหรับคู่รักแบบดูเร็ว | คู่มือย่านที่พักในโซลสำหรับคู่รัก เปรียบเทียบฮงแด ซองซู อินซาดง เมียงดง กังนัม และจัมชิลตามรูปแบบการเที่ยว | ฮงแด ซองซู อินซาดง เมียงดง กังนัม และจัมชิลเหมาะกับจังหวะทริปต่างกันมาก ตั้งแต่ค่ำคืนคึกคักไปจนถึงวันคาเฟ่ช้าๆ |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 46 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 91, 169 | 2 |

### ASSET-010 — couples-stay-seoul-area-guide-zh-tw.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `couples-stay-seoul-area-guide-zh-tw.png` |
| Extension | .png |
| Old Path | `images/Accommodation/couples-stay-seoul-area-guide-zh-tw.png` |
| New / Current Path | `images/Accommodation/couples-stay-seoul-area-guide-zh-tw.png` |
| SHA-256 | `64e0fc764c8d64d8f7cb23cb012c6df5e28b22742e616a4f8ce1c166d89fc3b3` |
| Dimensions | 1536 × 1024 |
| File Size | 2131838 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:99 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `zh-tw/best-area-for-couples-seoul.html` | 359 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `zh-tw/best-area-for-couples-seoul.html:359` | 首爾情侶住宿區快速比較 | 首爾情侶住宿區指南，依旅行風格比較弘大、聖水、仁寺洞、明洞、江南與蠶室。 | 從深夜活動到慢步調咖啡廳白天，弘大、聖水、仁寺洞、明洞、江南與蠶室各適合不同節奏。 |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-011 — couples-stay-seoul-area-guide.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `couples-stay-seoul-area-guide.png` |
| Extension | .png |
| Old Path | `images/Accommodation/couples-stay-seoul-area-guide.png` |
| New / Current Path | `images/Accommodation/couples-stay-seoul-area-guide.png` |
| SHA-256 | `184d0526b2c08d4cdc1c5c4b8a09591e1486a38180fb711caae1b40464a35a9c` |
| Dimensions | 1536 × 1024 |
| File Size | 2126092 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 3 |
| Runtime Reference Count | 3 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:99 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-couples-seoul.html` | 359 | 1 |
| `de/best-area-for-couples-seoul.html` | 359 | 1 |
| `fr/best-area-for-couples-seoul.html` | 359 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-couples-seoul.html:359` | Compare Seoul Areas for Couples at a Glance | Couples stay area guide for Seoul comparing Hongdae, Seongsu, Insadong, Myeongdong, Gangnam and Jamsil by travel style. | Hongdae, Seongsu, Insadong, Myeongdong, Gangnam and Jamsil suit very different rhythms, from late nights to slower café days. |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30.md` | 50, 1689 | 2 |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30_FINAL.md` | 51, 1686 | 2 |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 45 | 1 |
| `md/승인본/태국어/Korea_Inside_TH_Stay_Decision_Batch3_Localized_Review_2026-10-02.md` | 530 | 1 |
| `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | 99 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 80, 89 | 2 |

### ASSET-012 — dongdaemun-ddp-night.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `dongdaemun-ddp-night.png` |
| Extension | .png |
| Old Path | `images/Accommodation/dongdaemun-ddp-night.png` |
| New / Current Path | `images/Accommodation/dongdaemun-ddp-night.png` |
| SHA-256 | `8ce8811e29981bd75f497864cd7f49b342e02ed790d0a2a1d310bb8bcf0186e2` |
| Dimensions | 1408 × 768 |
| File Size | 2555594 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: DDP curved landmark and surrounding city roads at night. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-013 — dongdaemun-ddp-night.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `dongdaemun-ddp-night.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/dongdaemun-ddp-night.webp` |
| New / Current Path | `images/Accommodation/dongdaemun-ddp-night.webp` |
| SHA-256 | `13687980854c2e989cfe40d01f895a25f33ba289f70ede6f00ba165b4e861b28` |
| Dimensions | 1408 × 768 |
| File Size | 620604 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 21 |
| Runtime Reference Count | 21 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 499 | 1 |
| `best-area-for-families-seoul.html` | 583 | 1 |
| `best-area-for-shopping-seoul.html` | 469 | 1 |
| `de/accommodation.html` | 499 | 1 |
| `de/best-area-for-families-seoul.html` | 583 | 1 |
| `de/best-area-for-shopping-seoul.html` | 470 | 1 |
| `es/accommodation.html` | 391 | 1 |
| `es/best-area-for-families-seoul.html` | 451 | 1 |
| `es/best-area-for-shopping-seoul.html` | 377 | 1 |
| `fr/accommodation.html` | 499 | 1 |
| `fr/best-area-for-families-seoul.html` | 583 | 1 |
| `fr/best-area-for-shopping-seoul.html` | 469 | 1 |
| `ja/accommodation.html` | 499 | 1 |
| `ja/best-area-for-families-seoul.html` | 583 | 1 |
| `ja/best-area-for-shopping-seoul.html` | 469 | 1 |
| `th/accommodation.html` | 500 | 1 |
| `th/best-area-for-families-seoul.html` | 584 | 1 |
| `th/best-area-for-shopping-seoul.html` | 470 | 1 |
| `zh-tw/accommodation.html` | 499 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 583 | 1 |
| `zh-tw/best-area-for-shopping-seoul.html` | 469 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:499` | Dongdaemun | Dongdaemun Design Plaza (DDP) at night in Seoul | None |
| `best-area-for-families-seoul.html:583` | Dongdaemun | Dongdaemun Design Plaza (DDP) at night in Seoul | None |
| `best-area-for-shopping-seoul.html:469` | Dongdaemun | Dongdaemun Design Plaza (DDP) at night in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-014 — dongdaemun-gate-night.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `dongdaemun-gate-night.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/dongdaemun-gate-night.jpg` |
| New / Current Path | `images/Accommodation/dongdaemun-gate-night.jpg` |
| SHA-256 | `5c58cec06bf4e47cedfc6558773a056d113fc43bd5f3eb93611277a79e626f0d` |
| Dimensions | 5472 × 3648 |
| File Size | 2597401 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-first-time-visitors-seoul.html` | 580 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 580 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 440 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 580 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 580 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 581 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 580 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-first-time-visitors-seoul.html:580` | Dongdaemun | Heunginjimun Gate in Dongdaemun at night | Photo: Korea Tourism Organization / Lee Beom-su |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-015 — family-stay-seoul-area-guide-es.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `family-stay-seoul-area-guide-es.png` |
| Extension | .png |
| Old Path | `images/Accommodation/family-stay-seoul-area-guide-es.png` |
| New / Current Path | `images/Accommodation/family-stay-seoul-area-guide-es.png` |
| SHA-256 | `855ee0dfb182304557667e30c975ce76922ef1aa3f11679e577790ae285f944f` |
| Dimensions | 1491 × 1055 |
| File Size | 1657602 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:112 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `es/best-area-for-families-seoul.html` | 359 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `es/best-area-for-families-seoul.html:358` | Zonas para familias de un vistazo | Guía de zonas de alojamiento familiar en Seoul que compara Myeongdong, Jamsil, Mapo y Gongdeok, Insadong, Seoul Station y Hongdae. | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 309, 309 | 2 |

### ASSET-016 — family-stay-seoul-area-guide-ja.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `family-stay-seoul-area-guide-ja.png` |
| Extension | .png |
| Old Path | `images/Accommodation/family-stay-seoul-area-guide-ja.png` |
| New / Current Path | `images/Accommodation/family-stay-seoul-area-guide-ja.png` |
| SHA-256 | `73895edfdec4d3375eba18694ecb8cfd5b094b3c3ba04ae9ad3f5b500e347c15` |
| Dimensions | 1491 × 1055 |
| File Size | 1632557 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:112 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `ja/best-area-for-families-seoul.html` | 491 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `ja/best-area-for-families-seoul.html:490` | 家族向け宿泊エリア：ひと目で比較 | ソウルの家族向け宿泊エリアとして、明洞、蚕室、麻浦・孔徳、仁寺洞、ソウル駅、弘大を比較するガイド | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 310, 310 | 2 |

### ASSET-017 — family-stay-seoul-area-guide-th.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `family-stay-seoul-area-guide-th.png` |
| Extension | .png |
| Old Path | `images/Accommodation/family-stay-seoul-area-guide-th.png` |
| New / Current Path | `images/Accommodation/family-stay-seoul-area-guide-th.png` |
| SHA-256 | `b369cab50c27261173dfc2352b596f175be5a4e276e17d14c662239e0c4285d2` |
| Dimensions | 1491 × 1055 |
| File Size | 1688233 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:112 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `th/best-area-for-families-seoul.html` | 492 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `th/best-area-for-families-seoul.html:491` | ย่านสำหรับครอบครัวแบบดูเร็ว | คู่มือย่านที่พักในโซลสำหรับครอบครัว เปรียบเทียบเมียงดง จัมชิล มาโพและกงด็อก อินซาดง สถานีโซล และฮงแด | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 93 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 208, 300 | 2 |

### ASSET-018 — family-stay-seoul-area-guide-zh-tw.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `family-stay-seoul-area-guide-zh-tw.png` |
| Extension | .png |
| Old Path | `images/Accommodation/family-stay-seoul-area-guide-zh-tw.png` |
| New / Current Path | `images/Accommodation/family-stay-seoul-area-guide-zh-tw.png` |
| SHA-256 | `fe026cc8baaaad5d473694f5f98bf095a828fc3eed1463e00eab3a4fbf150ed9` |
| Dimensions | 1491 × 1055 |
| File Size | 1732695 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:112 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `zh-tw/best-area-for-families-seoul.html` | 491 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `zh-tw/best-area-for-families-seoul.html:490` | 家庭住宿區快速比較 | 首爾家庭住宿區指南，比較明洞、蠶室、麻浦與孔德、仁寺洞、首爾站和弘大。 | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-019 — family-stay-seoul-area-guide.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `family-stay-seoul-area-guide.png` |
| Extension | .png |
| Old Path | `images/Accommodation/family-stay-seoul-area-guide.png` |
| New / Current Path | `images/Accommodation/family-stay-seoul-area-guide.png` |
| SHA-256 | `5b241eb23e123238e527694cf2d3e377dac5244b355d661ec180b4f2fd5442a1` |
| Dimensions | 1491 × 1055 |
| File Size | 1574627 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 3 |
| Runtime Reference Count | 3 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:112 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-families-seoul.html` | 491 | 1 |
| `de/best-area-for-families-seoul.html` | 491 | 1 |
| `fr/best-area-for-families-seoul.html` | 491 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-families-seoul.html:490` | Family stay areas at a glance | Family accommodation area guide for Seoul comparing Myeongdong, Jamsil, Mapo and Gongdeok, Insadong, Seoul Station, and Hongdae. | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30.md` | 358, 1690 | 2 |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30_FINAL.md` | 358, 1687 | 2 |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 92 | 1 |
| `md/승인본/태국어/Korea_Inside_TH_Stay_Decision_Batch3_Localized_Review_2026-10-02.md` | 53 | 1 |
| `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | 112 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 197, 206 | 2 |

### ASSET-020 — first-time-seoul-area-guide-es.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `first-time-seoul-area-guide-es.png` |
| Extension | .png |
| Old Path | `images/Accommodation/first-time-seoul-area-guide-es.png` |
| New / Current Path | `images/Accommodation/first-time-seoul-area-guide-es.png` |
| SHA-256 | `29631b8fad10a57f0b234969562ed434b777d2c1666b4aacf16b0b3aa1345cb6` |
| Dimensions | 1536 × 1024 |
| File Size | 2405022 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:125 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `es/best-area-for-first-time-visitors-seoul.html` | 229 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `es/best-area-for-first-time-visitors-seoul.html:229` | Compras, comidas y comodidad diaria | Guía de zonas para alojarse en Seoul en una primera visita que relaciona turismo, vida nocturna, tren al aeropuerto, equipaje, calles tradicionales y planes en Lotte World con seis barrios prácticos. | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 430, 430 | 2 |

### ASSET-021 — first-time-seoul-area-guide-ja.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `first-time-seoul-area-guide-ja.png` |
| Extension | .png |
| Old Path | `images/Accommodation/first-time-seoul-area-guide-ja.png` |
| New / Current Path | `images/Accommodation/first-time-seoul-area-guide-ja.png` |
| SHA-256 | `dce63451afb87525fe26f995b475014c94699e41e0418a36b7607193621c797a` |
| Dimensions | 1536 × 1024 |
| File Size | 2378157 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:125 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `ja/best-area-for-first-time-visitors-seoul.html` | 369 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `ja/best-area-for-first-time-visitors-seoul.html:369` | 買い物・食事・日常の便利さ | 初めてのソウル旅行で、観光、ナイトライフ、空港鉄道、荷物、伝統的な街並み、ロッテワールドの予定に合わせて6つの実用的な宿泊エリアを比較するガイド | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 431, 431 | 2 |

### ASSET-022 — first-time-seoul-area-guide-th.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `first-time-seoul-area-guide-th.png` |
| Extension | .png |
| Old Path | `images/Accommodation/first-time-seoul-area-guide-th.png` |
| New / Current Path | `images/Accommodation/first-time-seoul-area-guide-th.png` |
| SHA-256 | `4b4c3d83ee2f56f5ce4fca36cdb0e26509c47fb5403979d839b119ed1d658ed9` |
| Dimensions | 1536 × 1024 |
| File Size | 2327911 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 2 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:125 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `images/Accommodation/infographic-localization-template-2026-10-02.json` | 45 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 370 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `th/best-area-for-first-time-visitors-seoul.html:370` | ช้อปปิ้ง อาหาร และความสะดวกในแต่ละวัน | คู่มือย่านที่พักในโซลสำหรับทริปแรก จับคู่การเที่ยว ชีวิตกลางคืน รถไฟสนามบิน กระเป๋า ถนนดั้งเดิม และแผน Lotte World กับย่านที่ใช้งานได้จริง 6 ย่าน | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 147 | 1 |
| `md/작업자료/Korea_Inside_Infographic_Localization_2_Families_Source_and_Copy_Gate_2026-10-02.md` | 26 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 339, 421 | 2 |

### ASSET-023 — first-time-seoul-area-guide-zh-tw.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `first-time-seoul-area-guide-zh-tw.png` |
| Extension | .png |
| Old Path | `images/Accommodation/first-time-seoul-area-guide-zh-tw.png` |
| New / Current Path | `images/Accommodation/first-time-seoul-area-guide-zh-tw.png` |
| SHA-256 | `cf434cc0616434c25a169990c2865d2e65bcb02333ba9e2e575d555dfa222cc2` |
| Dimensions | 1536 × 1024 |
| File Size | 2471566 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:125 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 369 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `zh-tw/best-area-for-first-time-visitors-seoul.html:369` | 購物、吃飯與日常便利 | 初訪首爾住宿區指南，依觀光、夜生活、機場鐵路、行李、傳統街道與 Lotte World 行程，搭配六個實用街區。 | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-024 — first-time-seoul-area-guide.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `first-time-seoul-area-guide.png` |
| Extension | .png |
| Old Path | `images/Accommodation/first-time-seoul-area-guide.png` |
| New / Current Path | `images/Accommodation/first-time-seoul-area-guide.png` |
| SHA-256 | `2e958e12213927ce0a4a15588c525875ec14323eaf04486ae4e468784d988421` |
| Dimensions | 1536 × 1024 |
| File Size | 2653378 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 3 |
| Runtime Reference Count | 4 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:125 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-first-time-visitors-seoul.html` | 369 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 369 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 369 | 1 |
| `images/Accommodation/infographic-localization-template-2026-10-02.json` | 25 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-first-time-visitors-seoul.html:369` | Shopping, meals and daily convenience | First-time Seoul stay area guide matching sightseeing, nightlife, airport rail, luggage, traditional streets and Lotte World plans with six practical neighborhoods. | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30.md` | 666, 1691 | 2 |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30_FINAL.md` | 665, 1688 | 2 |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 146 | 1 |
| `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | 125 | 1 |
| `md/작업자료/Korea_Inside_Infographic_Localization_2_Families_Source_and_Copy_Gate_2026-10-02.md` | 26 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 328, 337 | 2 |

### ASSET-025 — gangnam-city-2.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `gangnam-city-2.png` |
| Extension | .png |
| Old Path | `images/Accommodation/gangnam-city-2.png` |
| New / Current Path | `images/Accommodation/gangnam-city-2.png` |
| SHA-256 | `427751ca458da8e657b743b6ee43795c3c3a6338005e219a5f95cc90ad840504` |
| Dimensions | 1536 × 1024 |
| File Size | 2862288 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: broad urban street, pedestrians, Gangnam station sign and storefronts at dusk. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-026 — gangnam-city-2.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `gangnam-city-2.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/gangnam-city-2.webp` |
| New / Current Path | `images/Accommodation/gangnam-city-2.webp` |
| SHA-256 | `bccd2b8f181336c7a24d7d52cc93fdea78223df55cb06ec8686ac28e36659a3d` |
| Dimensions | 1536 × 1024 |
| File Size | 821274 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 6 |
| Runtime Reference Count | 6 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-nightlife-seoul.html` | 408 | 1 |
| `de/best-area-for-nightlife-seoul.html` | 408 | 1 |
| `es/best-area-for-nightlife-seoul.html` | 300 | 1 |
| `fr/best-area-for-nightlife-seoul.html` | 408 | 1 |
| `ja/best-area-for-nightlife-seoul.html` | 408 | 1 |
| `zh-tw/best-area-for-nightlife-seoul.html` | 408 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-nightlife-seoul.html:408` | Gangnam | Gangnam city streets at night in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-027 — gangnam-city.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `gangnam-city.png` |
| Extension | .png |
| Old Path | `images/Accommodation/gangnam-city.png` |
| New / Current Path | `images/Accommodation/gangnam-city.png` |
| SHA-256 | `05dcd637a49ca15b721d2fa5730b09cc0953c9e2376953c113f461956b8fe380` |
| Dimensions | 1536 × 1024 |
| File Size | 2825339 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: urban Gangnam street, traffic, station sign and commercial buildings. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-028 — gangnam-city.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `gangnam-city.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/gangnam-city.webp` |
| New / Current Path | `images/Accommodation/gangnam-city.webp` |
| SHA-256 | `78b6553ba0c45eee3f92012e31da48de9cc30e91fd676aa436d5b4fa5229ba03` |
| Dimensions | 1536 × 1024 |
| File Size | 777608 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 449 | 1 |
| `de/accommodation.html` | 449 | 1 |
| `es/accommodation.html` | 341 | 1 |
| `fr/accommodation.html` | 449 | 1 |
| `ja/accommodation.html` | 449 | 1 |
| `th/accommodation.html` | 450 | 1 |
| `zh-tw/accommodation.html` | 449 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:449` | Gangnam | Gangnam city streets in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-029 — gangnam-station-crossroads-kto.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `gangnam-station-crossroads-kto.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/gangnam-station-crossroads-kto.jpg` |
| New / Current Path | `images/Accommodation/gangnam-station-crossroads-kto.jpg` |
| SHA-256 | `a1a3d931d2645aa54ec347f512b6b684da339dc68665e725a7ddfba702777215` |
| Dimensions | 5000 × 3327 |
| File Size | 2532483 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 6 |
| Runtime Reference Count | 6 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-luxury-hotels-seoul.html` | 388 | 1 |
| `de/best-area-for-luxury-hotels-seoul.html` | 388 | 1 |
| `es/best-area-for-luxury-hotels-seoul.html` | 286 | 1 |
| `fr/best-area-for-luxury-hotels-seoul.html` | 388 | 1 |
| `ja/best-area-for-luxury-hotels-seoul.html` | 378 | 1 |
| `zh-tw/best-area-for-luxury-hotels-seoul.html` | 388 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-luxury-hotels-seoul.html:388` | Gangnam | Gangnam Station street in Seoul | Photo: Korea Tourism Organization / Live Studio Kim Hak-ri |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-030 — gangnam-station-street-family.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `gangnam-station-street-family.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/gangnam-station-street-family.jpg` |
| New / Current Path | `images/Accommodation/gangnam-station-street-family.jpg` |
| SHA-256 | `a1a3d931d2645aa54ec347f512b6b684da339dc68665e725a7ddfba702777215` |
| Dimensions | 5000 × 3327 |
| File Size | 2532483 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-families-seoul.html` | 612 | 1 |
| `de/best-area-for-families-seoul.html` | 612 | 1 |
| `es/best-area-for-families-seoul.html` | 480 | 1 |
| `fr/best-area-for-families-seoul.html` | 612 | 1 |
| `ja/best-area-for-families-seoul.html` | 612 | 1 |
| `th/best-area-for-families-seoul.html` | 613 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 612 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-families-seoul.html:612` | Gangnam | Street near Gangnam Station in Seoul | Photo: Korea Tourism Organization / Live Studio (Kim Hak-ri) |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-031 — gangnam-station-street.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `gangnam-station-street.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/gangnam-station-street.jpg` |
| New / Current Path | `images/Accommodation/gangnam-station-street.jpg` |
| SHA-256 | `ee5e16eddd1cf8a80da5cb180720be939688f07e316fd343e6e3019222f3eaa9` |
| Dimensions | 5000 × 3333 |
| File Size | 3089270 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 28 |
| Runtime Reference Count | 28 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-couples-seoul.html` | 432 | 1 |
| `best-area-for-first-time-visitors-seoul.html` | 565 | 1 |
| `best-area-for-shopping-seoul.html` | 433 | 1 |
| `best-area-for-solo-travelers-seoul.html` | 422 | 1 |
| `de/best-area-for-couples-seoul.html` | 432 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 565 | 1 |
| `de/best-area-for-shopping-seoul.html` | 434 | 1 |
| `de/best-area-for-solo-travelers-seoul.html` | 423 | 1 |
| `es/best-area-for-couples-seoul.html` | 336 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 425 | 1 |
| `es/best-area-for-shopping-seoul.html` | 341 | 1 |
| `es/best-area-for-solo-travelers-seoul.html` | 330 | 1 |
| `fr/best-area-for-couples-seoul.html` | 432 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 565 | 1 |
| `fr/best-area-for-shopping-seoul.html` | 433 | 1 |
| `fr/best-area-for-solo-travelers-seoul.html` | 422 | 1 |
| `ja/best-area-for-couples-seoul.html` | 428 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 565 | 1 |
| `ja/best-area-for-shopping-seoul.html` | 433 | 1 |
| `ja/best-area-for-solo-travelers-seoul.html` | 422 | 1 |
| `th/best-area-for-couples-seoul.html` | 433 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 566 | 1 |
| `th/best-area-for-shopping-seoul.html` | 434 | 1 |
| `th/best-area-for-solo-travelers-seoul.html` | 423 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 432 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 565 | 1 |
| `zh-tw/best-area-for-shopping-seoul.html` | 433 | 1 |
| `zh-tw/best-area-for-solo-travelers-seoul.html` | 422 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-couples-seoul.html:432` | Gangnam | Street near Gangnam Station in Seoul | Photo: Korea Tourism Organization / Live Studio (Kim Hak-ri) |
| `best-area-for-first-time-visitors-seoul.html:565` | Gangnam | Street near Gangnam Station in Seoul | Photo: Korea Tourism Organization / Live Studio (Kim Hak-ri) |
| `best-area-for-shopping-seoul.html:433` | Gangnam | Street near Gangnam Station in Seoul | Photo: Korea Tourism Organization / Live Studio (Kim Hak-ri) |
| `best-area-for-solo-travelers-seoul.html:422` | Gangnam | Street near Gangnam Station in Seoul | Photo: Korea Tourism Organization / Live Studio (Kim Hak-ri) |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-032 — holiday-inn-express-seoul-hongdae-exterior.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Holiday Inn Express Seoul Hongdae |
| Hotel Slug | holiday-inn-express-seoul-hongdae |
| Filename | `holiday-inn-express-seoul-hongdae-exterior.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/holiday-inn-express-seoul-hongdae-exterior.jpg` |
| New / Current Path | `images/Accommodation/hotels/holiday-inn-express-seoul-hongdae/holiday-inn-express-seoul-hongdae-exterior.jpg` |
| SHA-256 | `ab54a9e15e042c3f144c543c499a569862b2393c39adad612a03bd730b01242a` |
| Dimensions | 2133 × 1145 |
| File Size | 1286980 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:456; Airport &amp; AREX convenience → Holiday Inn Express Seoul Hongdae; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 456 | 1 |
| `es/where-to-stay-in-hongdae.html` | 456 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 456 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 456 | 1 |
| `th/where-to-stay-in-hongdae.html` | 457 | 1 |
| `where-to-stay-in-hongdae.html` | 456 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 456 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:456` | Holiday Inn Express Seoul Hongdae | Exterior of Holiday Inn Express Seoul Hongdae | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-033 — holiday-inn-express-seoul-hongdae-room.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Holiday Inn Express Seoul Hongdae |
| Hotel Slug | holiday-inn-express-seoul-hongdae |
| Filename | `holiday-inn-express-seoul-hongdae-room.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/holiday-inn-express-seoul-hongdae-room.jpg` |
| New / Current Path | `images/Accommodation/hotels/holiday-inn-express-seoul-hongdae/holiday-inn-express-seoul-hongdae-room.jpg` |
| SHA-256 | `ba6815318f4203bbca995871ea8db470da21471d170c761b264e2e65264aa6e0` |
| Dimensions | 684 × 483 |
| File Size | 379044 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:457; Airport &amp; AREX convenience → Holiday Inn Express Seoul Hongdae; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 457 | 1 |
| `es/where-to-stay-in-hongdae.html` | 457 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 457 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 457 | 1 |
| `th/where-to-stay-in-hongdae.html` | 458 | 1 |
| `where-to-stay-in-hongdae.html` | 457 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 457 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:457` | Holiday Inn Express Seoul Hongdae | Guest room at Holiday Inn Express Seoul Hongdae | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-034 — hongdae-busking.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-busking.png` |
| Extension | .png |
| Old Path | `images/Accommodation/hongdae-busking.png` |
| New / Current Path | `images/Accommodation/hongdae-busking.png` |
| SHA-256 | `a85dc458a35da91cd62ffda19fc95e5f08595682e358f41f94f5e9111725ad5e` |
| Dimensions | 1672 × 941 |
| File Size | 2895243 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: public pedestrian shopping avenue and street performers. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-035 — hongdae-busking.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-busking.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-busking.webp` |
| New / Current Path | `images/Accommodation/hongdae-busking.webp` |
| SHA-256 | `b28ef903ce46dc08a72ed739af45f231dca01d1900d03643052906e1cd929832` |
| Dimensions | 1672 × 941 |
| File Size | 906106 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 415 | 1 |
| `de/accommodation.html` | 415 | 1 |
| `es/accommodation.html` | 307 | 1 |
| `fr/accommodation.html` | 415 | 1 |
| `ja/accommodation.html` | 415 | 1 |
| `th/accommodation.html` | 416 | 1 |
| `zh-tw/accommodation.html` | 415 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:415` | Hongdae | Hongdae street performance area in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-036 — hongdae-busy-main-street.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-busy-main-street.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-busy-main-street.webp` |
| New / Current Path | `images/Accommodation/hongdae-busy-main-street.webp` |
| SHA-256 | `83105021c64bb46734231556a18fd85b705f76efbf658e820b160631a6531e49` |
| Dimensions | 5472 × 3648 |
| File Size | 1795874 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 42 |
| Runtime Reference Count | 42 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-couples-seoul.html` | 377 | 1 |
| `best-area-for-first-time-visitors-seoul.html` | 490 | 1 |
| `best-area-for-shopping-seoul.html` | 451 | 1 |
| `best-area-for-solo-travelers-seoul.html` | 362 | 1 |
| `de/best-area-for-couples-seoul.html` | 377 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 490 | 1 |
| `de/best-area-for-shopping-seoul.html` | 452 | 1 |
| `de/best-area-for-solo-travelers-seoul.html` | 363 | 1 |
| `de/hongdae-vs-myeongdong.html` | 546 | 1 |
| `de/where-to-stay-in-hongdae.html` | 223 | 1 |
| `es/best-area-for-couples-seoul.html` | 281 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 350 | 1 |
| `es/best-area-for-shopping-seoul.html` | 359 | 1 |
| `es/best-area-for-solo-travelers-seoul.html` | 270 | 1 |
| `es/hongdae-vs-myeongdong.html` | 454 | 1 |
| `es/where-to-stay-in-hongdae.html` | 223 | 1 |
| `fr/best-area-for-couples-seoul.html` | 377 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 490 | 1 |
| `fr/best-area-for-shopping-seoul.html` | 451 | 1 |
| `fr/best-area-for-solo-travelers-seoul.html` | 362 | 1 |
| `fr/hongdae-vs-myeongdong.html` | 546 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 223 | 1 |
| `hongdae-vs-myeongdong.html` | 546 | 1 |
| `ja/best-area-for-couples-seoul.html` | 373 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 490 | 1 |
| `ja/best-area-for-shopping-seoul.html` | 451 | 1 |
| `ja/best-area-for-solo-travelers-seoul.html` | 362 | 1 |
| `ja/hongdae-vs-myeongdong.html` | 546 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 223 | 1 |
| `th/best-area-for-couples-seoul.html` | 378 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 491 | 1 |
| `th/best-area-for-shopping-seoul.html` | 452 | 1 |
| `th/best-area-for-solo-travelers-seoul.html` | 363 | 1 |
| `th/hongdae-vs-myeongdong.html` | 547 | 1 |
| `th/where-to-stay-in-hongdae.html` | 224 | 1 |
| `where-to-stay-in-hongdae.html` | 223 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 377 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 490 | 1 |
| `zh-tw/best-area-for-shopping-seoul.html` | 451 | 1 |
| `zh-tw/best-area-for-solo-travelers-seoul.html` | 362 | 1 |
| `zh-tw/hongdae-vs-myeongdong.html` | 546 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 223 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-couples-seoul.html:377` | Hongdae | Busy shopping street in Hongdae, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-first-time-visitors-seoul.html:490` | Hongdae | Busy shopping street in Hongdae, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-shopping-seoul.html:451` | Hongdae | Busy shopping street in Hongdae, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-solo-travelers-seoul.html:362` | Hongdae | Busy shopping street in Hongdae, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `hongdae-vs-myeongdong.html:546` | What it’s like to stay in Hongdae | Busy shopping street in Hongdae, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `where-to-stay-in-hongdae.html:223` | Where to Stay in Hongdae: Areas, Hotels and Real Location Guide 2026 | Busy pedestrian shopping street in Hongdae, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-037 — hongdae-night-street.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-night-street.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-night-street.webp` |
| New / Current Path | `images/Accommodation/hongdae-night-street.webp` |
| SHA-256 | `e5bcc642bfc34f481649a9d348cb2e1a5ea3474c0d63dd2f66d8a000c3826192` |
| Dimensions | 5616 × 3744 |
| File Size | 866974 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 20 |
| Runtime Reference Count | 20 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-nightlife-seoul.html` | 379 | 1 |
| `de/best-area-for-nightlife-seoul.html` | 379 | 1 |
| `de/hongdae-vs-myeongdong.html` | 557 | 1 |
| `de/where-to-stay-in-hongdae.html` | 262 | 1 |
| `es/best-area-for-nightlife-seoul.html` | 271 | 1 |
| `es/hongdae-vs-myeongdong.html` | 465 | 1 |
| `es/where-to-stay-in-hongdae.html` | 262 | 1 |
| `fr/best-area-for-nightlife-seoul.html` | 379 | 1 |
| `fr/hongdae-vs-myeongdong.html` | 557 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 262 | 1 |
| `hongdae-vs-myeongdong.html` | 557 | 1 |
| `ja/best-area-for-nightlife-seoul.html` | 379 | 1 |
| `ja/hongdae-vs-myeongdong.html` | 557 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 262 | 1 |
| `th/hongdae-vs-myeongdong.html` | 558 | 1 |
| `th/where-to-stay-in-hongdae.html` | 263 | 1 |
| `where-to-stay-in-hongdae.html` | 262 | 1 |
| `zh-tw/best-area-for-nightlife-seoul.html` | 379 | 1 |
| `zh-tw/hongdae-vs-myeongdong.html` | 557 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 262 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-nightlife-seoul.html:379` | Hongdae | Hongdae street at night in Seoul | Photo: Korea Tourism Organization / Kim Ji-ho |
| `hongdae-vs-myeongdong.html:557` | What it’s like to stay in Hongdae | Hongdae street at night in Seoul | Photo: Korea Tourism Organization / Kim Ji-ho |
| `where-to-stay-in-hongdae.html:262` | Hongdae is not one single block | Hongdae street at night in Seoul | Photo: Korea Tourism Organization / Kim Ji-ho |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-038 — hongdae-tree-lined-street.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-tree-lined-street.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-tree-lined-street.webp` |
| New / Current Path | `images/Accommodation/hongdae-tree-lined-street.webp` |
| SHA-256 | `c93e19984eba7b842be445e4536239b42bd9d10818d8fd770e556d7f76d2314e` |
| Dimensions | 5472 × 3648 |
| File Size | 2484110 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 28 |
| Runtime Reference Count | 28 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-budget-travelers-seoul.html` | 424 | 1 |
| `best-area-for-families-seoul.html` | 597 | 1 |
| `de/best-area-for-budget-travelers-seoul.html` | 424 | 1 |
| `de/best-area-for-families-seoul.html` | 597 | 1 |
| `de/hongdae-vs-myeongdong.html` | 553 | 1 |
| `de/where-to-stay-in-hongdae.html` | 258 | 1 |
| `es/best-area-for-budget-travelers-seoul.html` | 298 | 1 |
| `es/best-area-for-families-seoul.html` | 465 | 1 |
| `es/hongdae-vs-myeongdong.html` | 461 | 1 |
| `es/where-to-stay-in-hongdae.html` | 258 | 1 |
| `fr/best-area-for-budget-travelers-seoul.html` | 424 | 1 |
| `fr/best-area-for-families-seoul.html` | 597 | 1 |
| `fr/hongdae-vs-myeongdong.html` | 553 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 258 | 1 |
| `hongdae-vs-myeongdong.html` | 553 | 1 |
| `ja/best-area-for-budget-travelers-seoul.html` | 424 | 1 |
| `ja/best-area-for-families-seoul.html` | 597 | 1 |
| `ja/hongdae-vs-myeongdong.html` | 553 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 258 | 1 |
| `th/best-area-for-budget-travelers-seoul.html` | 425 | 1 |
| `th/best-area-for-families-seoul.html` | 598 | 1 |
| `th/hongdae-vs-myeongdong.html` | 554 | 1 |
| `th/where-to-stay-in-hongdae.html` | 259 | 1 |
| `where-to-stay-in-hongdae.html` | 258 | 1 |
| `zh-tw/best-area-for-budget-travelers-seoul.html` | 424 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 597 | 1 |
| `zh-tw/hongdae-vs-myeongdong.html` | 553 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 258 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-budget-travelers-seoul.html:424` | Hongdae | Tree-lined street in the Hongdae area of Seoul | None |
| `best-area-for-families-seoul.html:597` | Hongdae | Tree-lined street in the Hongdae area of Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `hongdae-vs-myeongdong.html:553` | What it’s like to stay in Hongdae | Tree-lined street in the Hongdae area of Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `where-to-stay-in-hongdae.html:258` | Hongdae is not one single block | Tree-lined street in the Hongdae area of Seoul | Photo: Korea Tourism Organization / Lee Beom-su |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-039 — hongdae-vs-myeongdong-es.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-vs-myeongdong-es.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-vs-myeongdong-es.webp` |
| New / Current Path | `images/Accommodation/hongdae-vs-myeongdong-es.webp` |
| SHA-256 | `76bdce275be1744df0227678ac5e9d66b5724c27726d91d8c41b43296862bd42` |
| Dimensions | 1672 × 941 |
| File Size | 1740064 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:164 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `es/hongdae-vs-myeongdong.html` | 295 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `es/hongdae-vs-myeongdong.html:295` | La respuesta corta | Ilustración editorial que compara el ambiente nocturno de Hongdae y Myeongdong en Seoul. | Ilustración editorial que compara el ambiente nocturno de Hongdae y Myeongdong en Seoul. |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Remaining_Source_Extraction_2026-10-02.md` | 163, 164 | 2 |

### ASSET-040 — hongdae-vs-myeongdong-ja.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-vs-myeongdong-ja.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-vs-myeongdong-ja.webp` |
| New / Current Path | `images/Accommodation/hongdae-vs-myeongdong-ja.webp` |
| SHA-256 | `d7b545a576b30ee2b0ccdbf09177004449f33fa2acfd80bffac099403b58c1b4` |
| Dimensions | 1672 × 941 |
| File Size | 1740850 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:164 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `ja/hongdae-vs-myeongdong.html` | 387 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `ja/hongdae-vs-myeongdong.html:387` | まず結論 | ソウルの弘大と明洞の夜の雰囲気を比較した編集用イラスト | ソウルの弘大と明洞の夜の雰囲気を比較した編集用イラスト |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Remaining_Source_Extraction_2026-10-02.md` | 165, 166 | 2 |

### ASSET-041 — hongdae-vs-myeongdong-th.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-vs-myeongdong-th.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-vs-myeongdong-th.webp` |
| New / Current Path | `images/Accommodation/hongdae-vs-myeongdong-th.webp` |
| SHA-256 | `eb837dffc6b5a2d97f9047fcbdaf60d57ce50d4900eee149286a51a67ac50b8e` |
| Dimensions | 1672 × 941 |
| File Size | 1691794 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 2 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:164 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `images/Accommodation/infographic-localization-template-2026-10-02.json` | 60 | 1 |
| `th/hongdae-vs-myeongdong.html` | 388 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `th/hongdae-vs-myeongdong.html:388` | คำตอบแบบสั้น | ภาพประกอบเชิงบรรณาธิการเปรียบเทียบบรรยากาศช่วงค่ำของฮงแดและเมียงดงในโซล | ภาพประกอบเชิงบรรณาธิการเปรียบเทียบบรรยากาศช่วงค่ำของฮงแดและเมียงดงในโซล |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_Infographic_Localization_2_Families_Source_and_Copy_Gate_2026-10-02.md` | 27 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Remaining_Source_Extraction_2026-10-02.md` | 101, 157, 157, 159 | 4 |

### ASSET-042 — hongdae-vs-myeongdong-zh-tw.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-vs-myeongdong-zh-tw.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-vs-myeongdong-zh-tw.webp` |
| New / Current Path | `images/Accommodation/hongdae-vs-myeongdong-zh-tw.webp` |
| SHA-256 | `b596ce162548f65b6721fb0c0c86811a37edfecb618ece5010265692dbbf8601` |
| Dimensions | 1672 × 941 |
| File Size | 465500 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:164 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `zh-tw/hongdae-vs-myeongdong.html` | 387 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `zh-tw/hongdae-vs-myeongdong.html:387` | 先看重點 | 比較首爾弘大與明洞晚間氣氛的編輯插圖。 | 比較首爾弘大與明洞晚間氣氛的編輯插圖。 |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-043 — hongdae-vs-myeongdong.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `hongdae-vs-myeongdong.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/hongdae-vs-myeongdong.webp` |
| New / Current Path | `images/Accommodation/hongdae-vs-myeongdong.webp` |
| SHA-256 | `a1bdbff1f785e12489fc30bd3009f8c46648d5f3aba48e5dc314c4b7b6707537` |
| Dimensions | 1672 × 941 |
| File Size | 236616 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 3 |
| Runtime Reference Count | 4 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:164 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/hongdae-vs-myeongdong.html` | 387 | 1 |
| `fr/hongdae-vs-myeongdong.html` | 387 | 1 |
| `hongdae-vs-myeongdong.html` | 387 | 1 |
| `images/Accommodation/infographic-localization-template-2026-10-02.json` | 49 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `hongdae-vs-myeongdong.html:387` | The short answer | Editorial illustration comparing the evening atmosphere of Hongdae and Myeongdong in Seoul. | Editorial illustration comparing the evening atmosphere of Hongdae and Myeongdong in Seoul. |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | 164 | 1 |
| `md/작업자료/Korea_Inside_Infographic_Localization_2_Families_Source_and_Copy_Gate_2026-10-02.md` | 27 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Remaining_Source_Extraction_2026-10-02.md` | 137 | 1 |

### ASSET-044 — insadong-shopping-street-evening.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `insadong-shopping-street-evening.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/insadong-shopping-street-evening.jpg` |
| New / Current Path | `images/Accommodation/insadong-shopping-street-evening.jpg` |
| SHA-256 | `93396c41cd72098a745679af319721e0b4bcac5408dff709b620c5953b79a406` |
| Dimensions | 9328 × 6500 |
| File Size | 7594241 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 34 |
| Runtime Reference Count | 34 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-couples-seoul.html` | 404 | 1 |
| `best-area-for-families-seoul.html` | 554 | 1 |
| `best-area-for-first-time-visitors-seoul.html` | 535 | 1 |
| `best-area-for-luxury-hotels-seoul.html` | 450 | 1 |
| `best-area-for-solo-travelers-seoul.html` | 392 | 1 |
| `de/best-area-for-couples-seoul.html` | 404 | 1 |
| `de/best-area-for-families-seoul.html` | 554 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 535 | 1 |
| `de/best-area-for-luxury-hotels-seoul.html` | 450 | 1 |
| `de/best-area-for-solo-travelers-seoul.html` | 393 | 1 |
| `es/best-area-for-couples-seoul.html` | 308 | 1 |
| `es/best-area-for-families-seoul.html` | 422 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 395 | 1 |
| `es/best-area-for-luxury-hotels-seoul.html` | 348 | 1 |
| `es/best-area-for-solo-travelers-seoul.html` | 300 | 1 |
| `fr/best-area-for-couples-seoul.html` | 404 | 1 |
| `fr/best-area-for-families-seoul.html` | 554 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 535 | 1 |
| `fr/best-area-for-luxury-hotels-seoul.html` | 450 | 1 |
| `fr/best-area-for-solo-travelers-seoul.html` | 392 | 1 |
| `ja/best-area-for-couples-seoul.html` | 400 | 1 |
| `ja/best-area-for-families-seoul.html` | 554 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 535 | 1 |
| `ja/best-area-for-luxury-hotels-seoul.html` | 440 | 1 |
| `ja/best-area-for-solo-travelers-seoul.html` | 392 | 1 |
| `th/best-area-for-couples-seoul.html` | 405 | 1 |
| `th/best-area-for-families-seoul.html` | 555 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 536 | 1 |
| `th/best-area-for-solo-travelers-seoul.html` | 393 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 404 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 554 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 535 | 1 |
| `zh-tw/best-area-for-luxury-hotels-seoul.html` | 450 | 1 |
| `zh-tw/best-area-for-solo-travelers-seoul.html` | 392 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-couples-seoul.html:404` | Insadong | Shopping street in Insadong, Seoul | Photo: Korea Tourism Organization / Live Studio |
| `best-area-for-families-seoul.html:554` | Insadong | Shopping street in Insadong, Seoul | Photo: Korea Tourism Organization / Live Studio |
| `best-area-for-first-time-visitors-seoul.html:535` | Insadong | Shopping street in Insadong, Seoul | Photo: Korea Tourism Organization / Live Studio |
| `best-area-for-luxury-hotels-seoul.html:450` | Insadong | Insadong street view | Photo: Korea Tourism Organization / Live Studio |
| `best-area-for-solo-travelers-seoul.html:392` | Insadong | Shopping street in Insadong, Seoul | Photo: Korea Tourism Organization / Live Studio |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-045 — insadong-traditional-masks-.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `insadong-traditional-masks-.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/insadong-traditional-masks-.jpg` |
| New / Current Path | `images/Accommodation/insadong-traditional-masks-.jpg` |
| SHA-256 | `edea4b7159a3e32d26d4ed5a29ba03227666ada3964f1a4454b7d30ab63734a2` |
| Dimensions | 3264 × 2448 |
| File Size | 1971213 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: traditional masks on a shop display, matching the AREA cultural image role of the referenced WEBP. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-046 — insadong-traditional-masks-.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `insadong-traditional-masks-.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/insadong-traditional-masks-.webp` |
| New / Current Path | `images/Accommodation/insadong-traditional-masks-.webp` |
| SHA-256 | `da1f0acecd7451bc5705845ab7bc4e3d2d9598cde1075bd418b017af9b0342d9` |
| Dimensions | 3264 × 2448 |
| File Size | 2148020 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 466 | 1 |
| `de/accommodation.html` | 466 | 1 |
| `es/accommodation.html` | 358 | 1 |
| `fr/accommodation.html` | 466 | 1 |
| `ja/accommodation.html` | 466 | 1 |
| `th/accommodation.html` | 467 | 1 |
| `zh-tw/accommodation.html` | 466 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:466` | Insadong | Traditional masks in Insadong Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-047 — itaewon-night-street.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `itaewon-night-street.png` |
| Extension | .png |
| Old Path | `images/Accommodation/itaewon-night-street.png` |
| New / Current Path | `images/Accommodation/itaewon-night-street.png` |
| SHA-256 | `474656e47e06ee92e914fcb64def41f75602f18e6470a7cb8db7f8cf312ebf72` |
| Dimensions | 1536 × 1024 |
| File Size | 2545238 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: public Itaewon nightlife street with shops and restaurants, no hotel subject. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-048 — itaewon-night-street.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `itaewon-night-street.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/itaewon-night-street.webp` |
| New / Current Path | `images/Accommodation/itaewon-night-street.webp` |
| SHA-256 | `afc720670857117ac8ef5cbd1fa1d3722bdfb7ad635e18bd028bdae24a820c49` |
| Dimensions | 1536 × 1024 |
| File Size | 554852 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 26 |
| Runtime Reference Count | 26 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 550 | 1 |
| `best-area-for-couples-seoul.html` | 460 | 1 |
| `best-area-for-luxury-hotels-seoul.html` | 466 | 1 |
| `best-area-for-nightlife-seoul.html` | 394 | 1 |
| `de/accommodation.html` | 550 | 1 |
| `de/best-area-for-couples-seoul.html` | 460 | 1 |
| `de/best-area-for-luxury-hotels-seoul.html` | 466 | 1 |
| `de/best-area-for-nightlife-seoul.html` | 394 | 1 |
| `es/accommodation.html` | 442 | 1 |
| `es/best-area-for-couples-seoul.html` | 364 | 1 |
| `es/best-area-for-luxury-hotels-seoul.html` | 364 | 1 |
| `es/best-area-for-nightlife-seoul.html` | 286 | 1 |
| `fr/accommodation.html` | 550 | 1 |
| `fr/best-area-for-couples-seoul.html` | 460 | 1 |
| `fr/best-area-for-luxury-hotels-seoul.html` | 466 | 1 |
| `fr/best-area-for-nightlife-seoul.html` | 394 | 1 |
| `ja/accommodation.html` | 550 | 1 |
| `ja/best-area-for-couples-seoul.html` | 456 | 1 |
| `ja/best-area-for-luxury-hotels-seoul.html` | 456 | 1 |
| `ja/best-area-for-nightlife-seoul.html` | 394 | 1 |
| `th/accommodation.html` | 551 | 1 |
| `th/best-area-for-couples-seoul.html` | 461 | 1 |
| `zh-tw/accommodation.html` | 550 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 460 | 1 |
| `zh-tw/best-area-for-luxury-hotels-seoul.html` | 466 | 1 |
| `zh-tw/best-area-for-nightlife-seoul.html` | 394 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:550` | Itaewon | Itaewon nightlife street in Seoul | None |
| `best-area-for-couples-seoul.html:460` | Itaewon | Itaewon street at night in Seoul | None |
| `best-area-for-luxury-hotels-seoul.html:466` | Itaewon | Itaewon night street | AI-generated |
| `best-area-for-nightlife-seoul.html:394` | Itaewon | Itaewon nightlife street in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-049 — jamsil-lotte-world-tower-seokchon-lake.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `jamsil-lotte-world-tower-seokchon-lake.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/jamsil-lotte-world-tower-seokchon-lake.jpg` |
| New / Current Path | `images/Accommodation/jamsil-lotte-world-tower-seokchon-lake.jpg` |
| SHA-256 | `99cf132c2dbb98a028fa1e61b998f1f3e96eac9ff0222ec3b03df34c10c16db9` |
| Dimensions | 5472 × 3648 |
| File Size | 2680426 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 13 |
| Runtime Reference Count | 13 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-luxury-hotels-seoul.html` | 404 | 1 |
| `best-area-for-shopping-seoul.html` | 503 | 1 |
| `de/best-area-for-luxury-hotels-seoul.html` | 404 | 1 |
| `de/best-area-for-shopping-seoul.html` | 504 | 1 |
| `es/best-area-for-luxury-hotels-seoul.html` | 302 | 1 |
| `es/best-area-for-shopping-seoul.html` | 411 | 1 |
| `fr/best-area-for-luxury-hotels-seoul.html` | 404 | 1 |
| `fr/best-area-for-shopping-seoul.html` | 503 | 1 |
| `ja/best-area-for-luxury-hotels-seoul.html` | 394 | 1 |
| `ja/best-area-for-shopping-seoul.html` | 503 | 1 |
| `th/best-area-for-shopping-seoul.html` | 504 | 1 |
| `zh-tw/best-area-for-luxury-hotels-seoul.html` | 404 | 1 |
| `zh-tw/best-area-for-shopping-seoul.html` | 503 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-luxury-hotels-seoul.html:404` | Jamsil | Jamsil city view | None |
| `best-area-for-shopping-seoul.html:503` | Jamsil | Seokchon Lake and Lotte World Tower in Jamsil, Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-050 — jamsil-seokchon-lake-autumn.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `jamsil-seokchon-lake-autumn.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/jamsil-seokchon-lake-autumn.jpg` |
| New / Current Path | `images/Accommodation/jamsil-seokchon-lake-autumn.jpg` |
| SHA-256 | `b7cdc9c07dfbfcb914a8ca746ab5c7939910db20aa84539e79c3cbb094927640` |
| Dimensions | 4000 × 2673 |
| File Size | 1991519 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 21 |
| Runtime Reference Count | 21 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-couples-seoul.html` | 446 | 1 |
| `best-area-for-families-seoul.html` | 524 | 1 |
| `best-area-for-first-time-visitors-seoul.html` | 550 | 1 |
| `de/best-area-for-couples-seoul.html` | 446 | 1 |
| `de/best-area-for-families-seoul.html` | 524 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 550 | 1 |
| `es/best-area-for-couples-seoul.html` | 350 | 1 |
| `es/best-area-for-families-seoul.html` | 392 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 410 | 1 |
| `fr/best-area-for-couples-seoul.html` | 446 | 1 |
| `fr/best-area-for-families-seoul.html` | 524 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 550 | 1 |
| `ja/best-area-for-couples-seoul.html` | 442 | 1 |
| `ja/best-area-for-families-seoul.html` | 524 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 550 | 1 |
| `th/best-area-for-couples-seoul.html` | 447 | 1 |
| `th/best-area-for-families-seoul.html` | 525 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 551 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 446 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 524 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 550 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-couples-seoul.html:446` | Jamsil | Seokchon Lake and Lotte World Tower in Jamsil | Photo: Korea Tourism Organization / Kim Seung-rae |
| `best-area-for-families-seoul.html:524` | Jamsil | Seokchon Lake and Lotte World Tower in Jamsil, Seoul | Photo: Korea Tourism Organization / Kim Seung-rae |
| `best-area-for-first-time-visitors-seoul.html:550` | Jamsil | Seokchon Lake and Lotte World Tower in Jamsil | Photo: Korea Tourism Organization / Kim Seung-rae |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-051 — jamsil-seokchon-lake.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `jamsil-seokchon-lake.png` |
| Extension | .png |
| Old Path | `images/Accommodation/jamsil-seokchon-lake.png` |
| New / Current Path | `images/Accommodation/jamsil-seokchon-lake.png` |
| SHA-256 | `43c8ed165e39e41c6a2d9db0299fd8d1f96452bd9485cd0013999f810ee6e45b` |
| Dimensions | 1536 × 1024 |
| File Size | 2875582 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: lake, public park promenade and Jamsil landmarks. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-052 — jamsil-seokchon-lake.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `jamsil-seokchon-lake.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/jamsil-seokchon-lake.webp` |
| New / Current Path | `images/Accommodation/jamsil-seokchon-lake.webp` |
| SHA-256 | `6a06844f9b113f403be5becad8b1343786cea705f508ecc6f174c4d5412e8166` |
| Dimensions | 1536 × 1024 |
| File Size | 711400 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 516 | 1 |
| `de/accommodation.html` | 516 | 1 |
| `es/accommodation.html` | 408 | 1 |
| `fr/accommodation.html` | 516 | 1 |
| `ja/accommodation.html` | 516 | 1 |
| `th/accommodation.html` | 517 | 1 |
| `zh-tw/accommodation.html` | 516 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:516` | Jamsil | Seokchon Lake near Jamsil in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-053 — jsm-studio-hongdae-exterior.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | JSM Studio Hongdae |
| Hotel Slug | jsm-studio-hongdae |
| Filename | `jsm-studio-hongdae-exterior.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/jsm-studio-hongdae-exterior.webp` |
| New / Current Path | `images/Accommodation/hotels/jsm-studio-hongdae/jsm-studio-hongdae-exterior.webp` |
| SHA-256 | `9d73495b833f307c3c5623ac1a5424fdcb3f692791d34f7c6b28e13374a4d49b` |
| Dimensions | 2133 × 1145 |
| File Size | 1298010 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:357; Staying in Hongdae with 5–6 people → JSM Studio Hongdae; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 357 | 1 |
| `es/where-to-stay-in-hongdae.html` | 357 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 357 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 357 | 1 |
| `th/where-to-stay-in-hongdae.html` | 358 | 1 |
| `where-to-stay-in-hongdae.html` | 357 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 357 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:357` | JSM Studio Hongdae | Exterior of Paradisetel building where JSM Studio Hongdae is located | Photo: Kakao Map road view |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-054 — junibino-hotel-hongdae-exterior.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Junibino Hotel Hongdae |
| Hotel Slug | junibino-hotel-hongdae |
| Filename | `junibino-hotel-hongdae-exterior.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/junibino-hotel-hongdae-exterior.jpg` |
| New / Current Path | `images/Accommodation/hotels/junibino-hotel-hongdae/junibino-hotel-hongdae-exterior.jpg` |
| SHA-256 | `dbddd06d8671c6ea8b5f87c0b48f5920bc16ffcd3bdf8dfa42ee619579ba41ab` |
| Dimensions | 2133 × 1145 |
| File Size | 310053 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:521; Good-value stays → Junibino Hotel Hongdae; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 521 | 1 |
| `es/where-to-stay-in-hongdae.html` | 521 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 521 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 521 | 1 |
| `th/where-to-stay-in-hongdae.html` | 522 | 1 |
| `where-to-stay-in-hongdae.html` | 521 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 521 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:521` | Junibino Hotel Hongdae | Exterior of Junibino Hotel Hongdae | Photo: Kakao Map road view |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-055 — l7-hongdae-by-lotte-hotels-exterior.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | L7 Hongdae |
| Hotel Slug | l7-hongdae |
| Filename | `l7-hongdae-by-lotte-hotels-exterior.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/l7-hongdae-by-lotte-hotels-exterior.webp` |
| New / Current Path | `images/Accommodation/hotels/l7-hongdae/l7-hongdae-by-lotte-hotels-exterior.webp` |
| SHA-256 | `33c8aa2ff602900688b69ff47daedc4c42ad3c79a08cb0a1549e439e40fd8b5c` |
| Dimensions | 2133 × 1145 |
| File Size | 911430 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:409; Hongdae signature stays → L7 HONGDAE by LOTTE HOTELS; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 409 | 1 |
| `es/where-to-stay-in-hongdae.html` | 409 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 409 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 409 | 1 |
| `th/where-to-stay-in-hongdae.html` | 410 | 1 |
| `where-to-stay-in-hongdae.html` | 409 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 409 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:409` | L7 HONGDAE by LOTTE HOTELS | Exterior of L7 HONGDAE by LOTTE HOTELS in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-056 — l7-hongdae-by-lotte-hotels-room.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | L7 Hongdae |
| Hotel Slug | l7-hongdae |
| Filename | `l7-hongdae-by-lotte-hotels-room.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/l7-hongdae-by-lotte-hotels-room.jpg` |
| New / Current Path | `images/Accommodation/hotels/l7-hongdae/l7-hongdae-by-lotte-hotels-room.jpg` |
| SHA-256 | `ba1d69842617cbe2b4284f20c828fce2464288088e7ee796b28f237b05699c10` |
| Dimensions | 800 × 600 |
| File Size | 517878 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:410; Hongdae signature stays → L7 HONGDAE by LOTTE HOTELS; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 410 | 1 |
| `es/where-to-stay-in-hongdae.html` | 410 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 410 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 410 | 1 |
| `th/where-to-stay-in-hongdae.html` | 411 | 1 |
| `where-to-stay-in-hongdae.html` | 410 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 410 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:410` | L7 HONGDAE by LOTTE HOTELS | Guest room at L7 HONGDAE by LOTTE HOTELS | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-057 — luxury-stay-seoul-area-guide-es.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `luxury-stay-seoul-area-guide-es.png` |
| Extension | .png |
| Old Path | `images/Accommodation/luxury-stay-seoul-area-guide-es.png` |
| New / Current Path | `images/Accommodation/luxury-stay-seoul-area-guide-es.png` |
| SHA-256 | `2f15589c4f5ce28a48488fbe0b8e405a52e1a77c1b7e7d4d4c1a50ec489c16aa` |
| Dimensions | 1536 × 1024 |
| File Size | 2470290 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:138 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `es/best-area-for-luxury-hotels-seoul.html` | 194 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `es/best-area-for-luxury-hotels-seoul.html:194` | Las compras de lujo solo son útiles cuando encajan con el resto del día | Guía de zonas de alojamiento de lujo en Seoul que relaciona compras premium, comodidad moderna, turismo céntrico, tren al aeropuerto, estancias culturales y vida nocturna internacional con seis barrios. | Seis bases en Seoul ofrecen tipos muy distintos de estancia premium, desde compras y negocios hasta acceso a palacios, comodidad para el aeropuerto y ambiente nocturno. |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 553, 553 | 2 |

### ASSET-058 — luxury-stay-seoul-area-guide-ja.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `luxury-stay-seoul-area-guide-ja.png` |
| Extension | .png |
| Old Path | `images/Accommodation/luxury-stay-seoul-area-guide-ja.png` |
| New / Current Path | `images/Accommodation/luxury-stay-seoul-area-guide-ja.png` |
| SHA-256 | `af71193154b4368532595eecddc21a7ebb0020d4908961a69c6b536170a75d53` |
| Dimensions | 1536 × 1024 |
| File Size | 2442503 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:138 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `ja/best-area-for-luxury-hotels-seoul.html` | 286 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `ja/best-area-for-luxury-hotels-seoul.html:286` | 高級ブランドの買い物も、一日の動線に合ってこそ便利 | プレミアムショッピング、モダンな快適さ、中心部観光、空港鉄道、文化滞在、国際色のあるナイトライフを6つの街と結び付けたソウル高級ホテル宿泊エリアガイド | ソウルの6つの拠点では、買い物・ビジネスから王宮アクセス、空港の便利さ、夜の雰囲気まで、高級滞在の性格が大きく異なります。 |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 554, 554 | 2 |

### ASSET-059 — luxury-stay-seoul-area-guide-th.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `luxury-stay-seoul-area-guide-th.png` |
| Extension | .png |
| Old Path | `images/Accommodation/luxury-stay-seoul-area-guide-th.png` |
| New / Current Path | `images/Accommodation/luxury-stay-seoul-area-guide-th.png` |
| SHA-256 | `f374fe690ac539e7858eca4b649f6e923f4ff6f1e67546afc66b082a13290ec5` |
| Dimensions | 1536 × 1024 |
| File Size | 2275732 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:138 |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 196 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 544 | 1 |

### ASSET-060 — luxury-stay-seoul-area-guide-zh-tw.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `luxury-stay-seoul-area-guide-zh-tw.png` |
| Extension | .png |
| Old Path | `images/Accommodation/luxury-stay-seoul-area-guide-zh-tw.png` |
| New / Current Path | `images/Accommodation/luxury-stay-seoul-area-guide-zh-tw.png` |
| SHA-256 | `28aa8d1c3bd2751fb082c9ccb0071a4db7a591bfec5f27fd501394094ecd08a7` |
| Dimensions | 1536 × 1024 |
| File Size | 2608764 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:138 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `zh-tw/best-area-for-luxury-hotels-seoul.html` | 292 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `zh-tw/best-area-for-luxury-hotels-seoul.html:292` | 高端購物要和當天其他行程相配才方便 | 首爾高級住宿區指南，將高端購物、現代化舒適設施、市中心觀光、機場鐵路、文化住宿及國際夜生活需求，對應到六個街區。 | 首爾六個住宿區各有不同的高級住宿體驗，涵蓋購物、商務、宮殿交通、機場便利與夜間氣氛。 |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-061 — luxury-stay-seoul-area-guide.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `luxury-stay-seoul-area-guide.png` |
| Extension | .png |
| Old Path | `images/Accommodation/luxury-stay-seoul-area-guide.png` |
| New / Current Path | `images/Accommodation/luxury-stay-seoul-area-guide.png` |
| SHA-256 | `a88d4eeb4a1678a82136a1bdb8893859e99544c41152c884990735e5ad364935` |
| Dimensions | 1536 × 1024 |
| File Size | 2715896 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 3 |
| Runtime Reference Count | 3 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:138 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-luxury-hotels-seoul.html` | 292 | 1 |
| `de/best-area-for-luxury-hotels-seoul.html` | 292 | 1 |
| `fr/best-area-for-luxury-hotels-seoul.html` | 292 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-luxury-hotels-seoul.html:292` | Luxury shopping is only useful when it fits the rest of the day | Luxury Seoul stay area guide matching premium shopping, modern comfort, central sightseeing, airport rail, cultural stays and international nightlife with six neighborhoods. | Six Seoul bases offer very different kinds of premium stays, from shopping and business to palace access, airport convenience and evening atmosphere. |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30.md` | 1008, 1692 | 2 |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30_FINAL.md` | 1005, 1689 | 2 |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 195 | 1 |
| `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | 138 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 449, 458 | 2 |

### ASSET-062 — mapo-gongdeok-station.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `mapo-gongdeok-station.png` |
| Extension | .png |
| Old Path | `images/Accommodation/mapo-gongdeok-station.png` |
| New / Current Path | `images/Accommodation/mapo-gongdeok-station.png` |
| SHA-256 | `f61786c44c50a3aec3f4de5a6b566ed88b47c778c3bc91fae0235abc552075cc` |
| Dimensions | 1536 × 1024 |
| File Size | 2792924 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: Gongdeok station entrance and surrounding urban street. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-063 — mapo-gongdeok-station.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `mapo-gongdeok-station.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/mapo-gongdeok-station.webp` |
| New / Current Path | `images/Accommodation/mapo-gongdeok-station.webp` |
| SHA-256 | `762605994d47a6b08532b53b84b4d42b68c9b8a71426557e69454fc4556c714f` |
| Dimensions | 1536 × 1024 |
| File Size | 811116 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 20 |
| Runtime Reference Count | 20 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 567 | 1 |
| `best-area-for-budget-travelers-seoul.html` | 445 | 1 |
| `best-area-for-nightlife-seoul.html` | 436 | 1 |
| `de/accommodation.html` | 567 | 1 |
| `de/best-area-for-budget-travelers-seoul.html` | 445 | 1 |
| `de/best-area-for-nightlife-seoul.html` | 436 | 1 |
| `es/accommodation.html` | 459 | 1 |
| `es/best-area-for-budget-travelers-seoul.html` | 319 | 1 |
| `es/best-area-for-nightlife-seoul.html` | 328 | 1 |
| `fr/accommodation.html` | 567 | 1 |
| `fr/best-area-for-budget-travelers-seoul.html` | 445 | 1 |
| `fr/best-area-for-nightlife-seoul.html` | 436 | 1 |
| `ja/accommodation.html` | 567 | 1 |
| `ja/best-area-for-budget-travelers-seoul.html` | 445 | 1 |
| `ja/best-area-for-nightlife-seoul.html` | 436 | 1 |
| `th/accommodation.html` | 568 | 1 |
| `th/best-area-for-budget-travelers-seoul.html` | 446 | 1 |
| `zh-tw/accommodation.html` | 567 | 1 |
| `zh-tw/best-area-for-budget-travelers-seoul.html` | 445 | 1 |
| `zh-tw/best-area-for-nightlife-seoul.html` | 436 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:567` | Mapo / Gongdeok | Mapo Gongdeok station area in Seoul | None |
| `best-area-for-budget-travelers-seoul.html:445` | Mapo / Gongdeok | Mapo Gongdeok station area in Seoul | None |
| `best-area-for-nightlife-seoul.html:436` | Mapo / Gongdeok | Mapo Gongdeok station area in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-064 — mapo-pork-rib-street-day.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `mapo-pork-rib-street-day.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/mapo-pork-rib-street-day.jpg` |
| New / Current Path | `images/Accommodation/mapo-pork-rib-street-day.jpg` |
| SHA-256 | `5c9df72f4aaa4b8c5ea9ac52b08e5ae7ac343983d989962fa965b1fc43310fee` |
| Dimensions | 5472 × 3648 |
| File Size | 3949677 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 28 |
| Runtime Reference Count | 28 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-couples-seoul.html` | 473 | 1 |
| `best-area-for-families-seoul.html` | 539 | 1 |
| `best-area-for-first-time-visitors-seoul.html` | 520 | 1 |
| `best-area-for-solo-travelers-seoul.html` | 407 | 1 |
| `de/best-area-for-couples-seoul.html` | 473 | 1 |
| `de/best-area-for-families-seoul.html` | 539 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 520 | 1 |
| `de/best-area-for-solo-travelers-seoul.html` | 408 | 1 |
| `es/best-area-for-couples-seoul.html` | 377 | 1 |
| `es/best-area-for-families-seoul.html` | 407 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 380 | 1 |
| `es/best-area-for-solo-travelers-seoul.html` | 315 | 1 |
| `fr/best-area-for-couples-seoul.html` | 473 | 1 |
| `fr/best-area-for-families-seoul.html` | 539 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 520 | 1 |
| `fr/best-area-for-solo-travelers-seoul.html` | 407 | 1 |
| `ja/best-area-for-couples-seoul.html` | 469 | 1 |
| `ja/best-area-for-families-seoul.html` | 539 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 520 | 1 |
| `ja/best-area-for-solo-travelers-seoul.html` | 407 | 1 |
| `th/best-area-for-couples-seoul.html` | 474 | 1 |
| `th/best-area-for-families-seoul.html` | 540 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 521 | 1 |
| `th/best-area-for-solo-travelers-seoul.html` | 408 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 473 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 539 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 520 | 1 |
| `zh-tw/best-area-for-solo-travelers-seoul.html` | 407 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-couples-seoul.html:473` | Mapo / Gongdeok | Restaurant street in Mapo, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-families-seoul.html:539` | Mapo / Gongdeok | Restaurant street in Mapo, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-first-time-visitors-seoul.html:520` | Mapo / Gongdeok | Restaurant street in Mapo, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-solo-travelers-seoul.html:407` | Mapo / Gongdeok | Restaurant street in Mapo, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-065 — mercure-ambassador-seoul-hongdae-exterior.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Mercure Ambassador Seoul Hongdae |
| Hotel Slug | mercure-ambassador-seoul-hongdae |
| Filename | `mercure-ambassador-seoul-hongdae-exterior.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/mercure-ambassador-seoul-hongdae-exterior.jpg` |
| New / Current Path | `images/Accommodation/hotels/mercure-ambassador-seoul-hongdae/mercure-ambassador-seoul-hongdae-exterior.jpg` |
| SHA-256 | `0106478d3be54596df9a4beb93cfffd9a909dfe893c506cc66dcbd8f076c44ec` |
| Dimensions | 940 × 705 |
| File Size | 764614 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:475; Airport &amp; AREX convenience → Mercure Ambassador Seoul Hongdae; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 475 | 1 |
| `es/where-to-stay-in-hongdae.html` | 475 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 475 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 475 | 1 |
| `th/where-to-stay-in-hongdae.html` | 476 | 1 |
| `where-to-stay-in-hongdae.html` | 475 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 475 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:475` | Mercure Ambassador Seoul Hongdae | Exterior of Mercure Ambassador Seoul Hongdae | Photo: © Korea Tourism Organization |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-066 — mercure-ambassador-seoul-hongdae-room.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Mercure Ambassador Seoul Hongdae |
| Hotel Slug | mercure-ambassador-seoul-hongdae |
| Filename | `mercure-ambassador-seoul-hongdae-room.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/mercure-ambassador-seoul-hongdae-room.jpg` |
| New / Current Path | `images/Accommodation/hotels/mercure-ambassador-seoul-hongdae/mercure-ambassador-seoul-hongdae-room.jpg` |
| SHA-256 | `ef1ee16ac99bf31c9dd646708de60372c3385103ed3f766d42014517d621d482` |
| Dimensions | 975 × 643 |
| File Size | 211547 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:476; Airport &amp; AREX convenience → Mercure Ambassador Seoul Hongdae; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 476 | 1 |
| `es/where-to-stay-in-hongdae.html` | 476 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 476 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 476 | 1 |
| `th/where-to-stay-in-hongdae.html` | 477 | 1 |
| `where-to-stay-in-hongdae.html` | 476 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 476 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:476` | Mercure Ambassador Seoul Hongdae | Guest room at Mercure Ambassador Seoul Hongdae | Photo: © Korea Tourism Organization |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-067 — myeongdong-crowded-shopping-street.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `myeongdong-crowded-shopping-street.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/myeongdong-crowded-shopping-street.webp` |
| New / Current Path | `images/Accommodation/myeongdong-crowded-shopping-street.webp` |
| SHA-256 | `5c5d0bf25ac6bb98ea2023a436a65263528f3cdfa6d9db8c89085b32c74220d1` |
| Dimensions | 876 × 584 |
| File Size | 230968 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/hongdae-vs-myeongdong.html` | 581 | 1 |
| `es/hongdae-vs-myeongdong.html` | 489 | 1 |
| `fr/hongdae-vs-myeongdong.html` | 581 | 1 |
| `hongdae-vs-myeongdong.html` | 581 | 1 |
| `ja/hongdae-vs-myeongdong.html` | 581 | 1 |
| `th/hongdae-vs-myeongdong.html` | 582 | 1 |
| `zh-tw/hongdae-vs-myeongdong.html` | 581 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `hongdae-vs-myeongdong.html:581` | What it’s like to stay in Myeongdong | Crowds walking between shops and digital displays in Myeongdong, Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-068 — myeongdong-day-shopping-street.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `myeongdong-day-shopping-street.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/myeongdong-day-shopping-street.webp` |
| New / Current Path | `images/Accommodation/myeongdong-day-shopping-street.webp` |
| SHA-256 | `645ab79ff627a24246e376cb641ca109d5b2a3e77a88bd44621375b944cb316b` |
| Dimensions | 5472 × 3648 |
| File Size | 2076776 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 41 |
| Runtime Reference Count | 41 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-couples-seoul.html` | 418 | 1 |
| `best-area-for-families-seoul.html` | 509 | 1 |
| `best-area-for-first-time-visitors-seoul.html` | 474 | 1 |
| `best-area-for-luxury-hotels-seoul.html` | 419 | 1 |
| `best-area-for-solo-travelers-seoul.html` | 347 | 1 |
| `de/best-area-for-couples-seoul.html` | 418 | 1 |
| `de/best-area-for-families-seoul.html` | 509 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 474 | 1 |
| `de/best-area-for-luxury-hotels-seoul.html` | 419 | 1 |
| `de/best-area-for-solo-travelers-seoul.html` | 348 | 1 |
| `de/hongdae-vs-myeongdong.html` | 575 | 1 |
| `es/best-area-for-couples-seoul.html` | 322 | 1 |
| `es/best-area-for-families-seoul.html` | 377 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 334 | 1 |
| `es/best-area-for-luxury-hotels-seoul.html` | 317 | 1 |
| `es/best-area-for-solo-travelers-seoul.html` | 255 | 1 |
| `es/hongdae-vs-myeongdong.html` | 483 | 1 |
| `fr/best-area-for-couples-seoul.html` | 418 | 1 |
| `fr/best-area-for-families-seoul.html` | 509 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 474 | 1 |
| `fr/best-area-for-luxury-hotels-seoul.html` | 419 | 1 |
| `fr/best-area-for-solo-travelers-seoul.html` | 347 | 1 |
| `fr/hongdae-vs-myeongdong.html` | 575 | 1 |
| `hongdae-vs-myeongdong.html` | 575 | 1 |
| `ja/best-area-for-couples-seoul.html` | 414 | 1 |
| `ja/best-area-for-families-seoul.html` | 509 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 474 | 1 |
| `ja/best-area-for-luxury-hotels-seoul.html` | 409 | 1 |
| `ja/best-area-for-solo-travelers-seoul.html` | 347 | 1 |
| `ja/hongdae-vs-myeongdong.html` | 575 | 1 |
| `th/best-area-for-couples-seoul.html` | 419 | 1 |
| `th/best-area-for-families-seoul.html` | 510 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 475 | 1 |
| `th/best-area-for-solo-travelers-seoul.html` | 348 | 1 |
| `th/hongdae-vs-myeongdong.html` | 576 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 418 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 509 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 474 | 1 |
| `zh-tw/best-area-for-luxury-hotels-seoul.html` | 419 | 1 |
| `zh-tw/best-area-for-solo-travelers-seoul.html` | 347 | 1 |
| `zh-tw/hongdae-vs-myeongdong.html` | 575 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-couples-seoul.html:418` | Myeongdong | Myeongdong shopping street in central Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-families-seoul.html:509` | Myeongdong | Myeongdong shopping street in central Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-first-time-visitors-seoul.html:474` | Myeongdong | Myeongdong shopping street in central Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-luxury-hotels-seoul.html:419` | Myeongdong | Myeongdong shopping street | Photo: Korea Tourism Organization / Lee Beom-su |
| `best-area-for-solo-travelers-seoul.html:347` | Myeongdong | Myeongdong shopping street in central Seoul | Photo: Korea Tourism Organization / Lee Beom-su |
| `hongdae-vs-myeongdong.html:575` | What it’s like to stay in Myeongdong | Pedestrian shopping street in Myeongdong, Seoul | Photo: Korea Tourism Organization / Lee Beom-su |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-069 — myeongdong-night-cityscape.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `myeongdong-night-cityscape.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/myeongdong-night-cityscape.webp` |
| New / Current Path | `images/Accommodation/myeongdong-night-cityscape.webp` |
| SHA-256 | `c62fb14f752e4d19acf03595d6c7d3dc42bf903aa4b1661d4170db36df0ec0a6` |
| Dimensions | 5000 × 2989 |
| File Size | 1061704 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 6 |
| Runtime Reference Count | 6 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-nightlife-seoul.html` | 422 | 1 |
| `de/best-area-for-nightlife-seoul.html` | 422 | 1 |
| `es/best-area-for-nightlife-seoul.html` | 314 | 1 |
| `fr/best-area-for-nightlife-seoul.html` | 422 | 1 |
| `ja/best-area-for-nightlife-seoul.html` | 422 | 1 |
| `zh-tw/best-area-for-nightlife-seoul.html` | 422 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-nightlife-seoul.html:422` | Myeongdong | Myeongdong cityscape at night in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-070 — myeongdong-shopping-street.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `myeongdong-shopping-street.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/myeongdong-shopping-street.jpg` |
| New / Current Path | `images/Accommodation/myeongdong-shopping-street.jpg` |
| SHA-256 | `fea72b029b68e84ffc598012b29593b163bf6cc8bf7c7087e307d65ecd842c81` |
| Dimensions | 5184 × 3456 |
| File Size | 2223099 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: public pedestrian shopping street and storefronts, no hotel card subject. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-071 — myeongdong-shopping-street.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `myeongdong-shopping-street.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/myeongdong-shopping-street.webp` |
| New / Current Path | `images/Accommodation/myeongdong-shopping-street.webp` |
| SHA-256 | `db9e55906362ec9ae7746a751ae4ccfb6129ffac20e1eb67d138874043a47574` |
| Dimensions | 5184 × 3456 |
| File Size | 2404020 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 14 |
| Runtime Reference Count | 14 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 432 | 1 |
| `best-area-for-shopping-seoul.html` | 416 | 1 |
| `de/accommodation.html` | 432 | 1 |
| `de/best-area-for-shopping-seoul.html` | 417 | 1 |
| `es/accommodation.html` | 324 | 1 |
| `es/best-area-for-shopping-seoul.html` | 324 | 1 |
| `fr/accommodation.html` | 432 | 1 |
| `fr/best-area-for-shopping-seoul.html` | 416 | 1 |
| `ja/accommodation.html` | 432 | 1 |
| `ja/best-area-for-shopping-seoul.html` | 416 | 1 |
| `th/accommodation.html` | 433 | 1 |
| `th/best-area-for-shopping-seoul.html` | 417 | 1 |
| `zh-tw/accommodation.html` | 432 | 1 |
| `zh-tw/best-area-for-shopping-seoul.html` | 416 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:432` | Myeongdong | Myeongdong shopping street in Seoul | None |
| `best-area-for-shopping-seoul.html:416` | Myeongdong | Myeongdong shopping street in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-072 — nightlife-stay-seoul-area-guide-es.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `nightlife-stay-seoul-area-guide-es.png` |
| Extension | .png |
| Old Path | `images/Accommodation/nightlife-stay-seoul-area-guide-es.png` |
| New / Current Path | `images/Accommodation/nightlife-stay-seoul-area-guide-es.png` |
| SHA-256 | `854c7ff85b9087e5b2600639a18edf949c36f337aa21a5f6cc34a006737174d6` |
| Dimensions | 1536 × 1024 |
| File Size | 1670424 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:151 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `es/best-area-for-nightlife-seoul.html` | 191 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `es/best-area-for-nightlife-seoul.html:191` | Compara las zonas de vida nocturna de Seoul de un vistazo | Guía en seis partes que relaciona las prioridades de un viaje nocturno por Seoul con Hongdae, Itaewon, Gangnam, Myeongdong, Mapo o Gongdeok y Seoul Station. | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 646, 646 | 2 |

### ASSET-073 — nightlife-stay-seoul-area-guide-ja.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `nightlife-stay-seoul-area-guide-ja.png` |
| Extension | .png |
| Old Path | `images/Accommodation/nightlife-stay-seoul-area-guide-ja.png` |
| New / Current Path | `images/Accommodation/nightlife-stay-seoul-area-guide-ja.png` |
| SHA-256 | `f45776a5982cf7d3669be26518a0dc9f451471ec3155c9759fd62fa30162e33d` |
| Dimensions | 1536 × 1024 |
| File Size | 1602659 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:151 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `ja/best-area-for-nightlife-seoul.html` | 299 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `ja/best-area-for-nightlife-seoul.html:299` | ソウルのナイトライフ宿泊エリアを一覧比較 | 弘大、梨泰院、江南、明洞、麻浦・孔徳、ソウル駅を、ソウルのナイトライフ旅行の優先条件別に比較する6エリアガイド | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 647, 647 | 2 |

### ASSET-074 — nightlife-stay-seoul-area-guide-th.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `nightlife-stay-seoul-area-guide-th.png` |
| Extension | .png |
| Old Path | `images/Accommodation/nightlife-stay-seoul-area-guide-th.png` |
| New / Current Path | `images/Accommodation/nightlife-stay-seoul-area-guide-th.png` |
| SHA-256 | `1354932d3148dccb94e007e70843ef067934b3c92524fe3c9a823d120c52b057` |
| Dimensions | 1536 × 1024 |
| File Size | 1594881 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:151 |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 246 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 637 | 1 |

### ASSET-075 — nightlife-stay-seoul-area-guide-zh-tw.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `nightlife-stay-seoul-area-guide-zh-tw.png` |
| Extension | .png |
| Old Path | `images/Accommodation/nightlife-stay-seoul-area-guide-zh-tw.png` |
| New / Current Path | `images/Accommodation/nightlife-stay-seoul-area-guide-zh-tw.png` |
| SHA-256 | `63eacd5a3484754efc75c1306c1b564855a0864938b15f1663b00cdcd2a65b3e` |
| Dimensions | 1536 × 1024 |
| File Size | 1637809 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 1 |
| Runtime Reference Count | 1 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:151 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `zh-tw/best-area-for-nightlife-seoul.html` | 299 | 1 |

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. Localized sibling context below records the existing alt/caption.

| Sibling page / line | Heading | Alt | Caption |
| --- | --- | --- | --- |
| `zh-tw/best-area-for-nightlife-seoul.html:299` | 首爾夜生活住宿區快速比較 | 六部分指南，依首爾夜生活旅行需求，搭配弘大、梨泰院、江南、明洞、麻浦或孔德，以及首爾站。 | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-076 — nightlife-stay-seoul-area-guide.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `nightlife-stay-seoul-area-guide.png` |
| Extension | .png |
| Old Path | `images/Accommodation/nightlife-stay-seoul-area-guide.png` |
| New / Current Path | `images/Accommodation/nightlife-stay-seoul-area-guide.png` |
| SHA-256 | `ae765cc088069bc9bd156b6e8f9b45e564d08ad6d4e460cfb39249401c2b5a2f` |
| Dimensions | 1536 × 1024 |
| File Size | 1590261 bytes |
| Classification | INFOGRAPHIC |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | INFOGRAPHIC |
| Confidence | HIGH |
| Status | INFOGRAPHIC_NOT_MOVED |
| HTML Referencing Pages | 3 |
| Runtime Reference Count | 3 |
| Attribution Basis | Existing ES/JA infographic audit identifies this base asset family (INF-001 to INF-006). Current localized sibling img alt/section identifies it as area guide or editorial comparison. English couples/family/first-time and comparison base files plus unreferenced Thai luxury/nightlife assets were visually inspected: editorial guide/comparison layout and embedded text, no hotel-specific body photo. |
| Existing audit evidence | md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md:151 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-nightlife-seoul.html` | 299 | 1 |
| `de/best-area-for-nightlife-seoul.html` | 299 | 1 |
| `fr/best-area-for-nightlife-seoul.html` | 299 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-nightlife-seoul.html:299` | Compare Seoul Nightlife Areas at a Glance | Six-part guide matching Seoul nightlife travel priorities with Hongdae, Itaewon, Gangnam, Myeongdong, Mapo or Gongdeok, and Seoul Station. | None |

Historical reference (existing Markdown left unchanged):

| Historical MD | Pre-move lines | Occurrences |
| --- | --- | --- |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30.md` | 1352, 1693 | 2 |
| `md/승인본/독일어/Korea_Inside_DE_Localization_Final_Review_Batch01C_5pages_2026-09-30_FINAL.md` | 1348, 1690 | 2 |
| `md/승인본/태국어/Korea_Inside_TH_Infographic_Batch1_Localized_Review_2026-10-02.md` | 245 | 1 |
| `md/작업자료/Korea_Inside_ES_JA_Infographic_English_Text_Audit_2026-09-27.md` | 151 | 1 |
| `md/작업자료/Korea_Inside_TH_Infographic_Batch1_Source_Extraction_2026-10-02.md` | 572, 581 | 2 |

### ASSET-077 — ryse-autograph-collection-exterior.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | RYSE, Autograph Collection Seoul |
| Hotel Slug | ryse-autograph-collection-seoul |
| Filename | `ryse-autograph-collection-exterior.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/ryse-autograph-collection-exterior.webp` |
| New / Current Path | `images/Accommodation/hotels/ryse-autograph-collection-seoul/ryse-autograph-collection-exterior.webp` |
| SHA-256 | `844afd01c91010edf1295202a934b42921d41e55698c7f6882a16120d1f3a8b1` |
| Dimensions | 2133 × 1145 |
| File Size | 1063412 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:428; Hongdae signature stays → RYSE, Autograph Collection; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 428 | 1 |
| `es/where-to-stay-in-hongdae.html` | 428 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 428 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 428 | 1 |
| `th/where-to-stay-in-hongdae.html` | 429 | 1 |
| `where-to-stay-in-hongdae.html` | 428 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 428 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:428` | RYSE, Autograph Collection | Exterior of RYSE, Autograph Collection in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-078 — ryse-autograph-collection-room.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | RYSE, Autograph Collection Seoul |
| Hotel Slug | ryse-autograph-collection-seoul |
| Filename | `ryse-autograph-collection-room.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/ryse-autograph-collection-room.jpg` |
| New / Current Path | `images/Accommodation/hotels/ryse-autograph-collection-seoul/ryse-autograph-collection-room.jpg` |
| SHA-256 | `9321b7767c7e659d4b67d16a8796b668557eaa266a58d7be475153d1227732ec` |
| Dimensions | 1024 × 803 |
| File Size | 1041512 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:429; Hongdae signature stays → RYSE, Autograph Collection; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 429 | 1 |
| `es/where-to-stay-in-hongdae.html` | 429 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 429 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 429 | 1 |
| `th/where-to-stay-in-hongdae.html` | 430 | 1 |
| `where-to-stay-in-hongdae.html` | 429 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 429 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:429` | RYSE, Autograph Collection | Guest room at RYSE, Autograph Collection | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-079 — seongsu-alley.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `seongsu-alley.png` |
| Extension | .png |
| Old Path | `images/Accommodation/seongsu-alley.png` |
| New / Current Path | `images/Accommodation/seongsu-alley.png` |
| SHA-256 | `54a13361dd622ca581a492b0e29c21e78bae9ce8f9523d46d2bbf598a8a014e8` |
| Dimensions | 1536 × 1024 |
| File Size | 3080122 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: public alley, cafes, pedestrians and storefronts. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-080 — seongsu-alley.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `seongsu-alley.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/seongsu-alley.webp` |
| New / Current Path | `images/Accommodation/seongsu-alley.webp` |
| SHA-256 | `f2823d3f02c805889db10d3914c9179de9ef6a2d0c01f121fe3f24ef07673a6e` |
| Dimensions | 1536 × 1024 |
| File Size | 882836 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 21 |
| Runtime Reference Count | 21 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 533 | 1 |
| `best-area-for-couples-seoul.html` | 391 | 1 |
| `best-area-for-shopping-seoul.html` | 486 | 1 |
| `de/accommodation.html` | 533 | 1 |
| `de/best-area-for-couples-seoul.html` | 391 | 1 |
| `de/best-area-for-shopping-seoul.html` | 487 | 1 |
| `es/accommodation.html` | 425 | 1 |
| `es/best-area-for-couples-seoul.html` | 295 | 1 |
| `es/best-area-for-shopping-seoul.html` | 394 | 1 |
| `fr/accommodation.html` | 533 | 1 |
| `fr/best-area-for-couples-seoul.html` | 391 | 1 |
| `fr/best-area-for-shopping-seoul.html` | 486 | 1 |
| `ja/accommodation.html` | 533 | 1 |
| `ja/best-area-for-couples-seoul.html` | 387 | 1 |
| `ja/best-area-for-shopping-seoul.html` | 486 | 1 |
| `th/accommodation.html` | 534 | 1 |
| `th/best-area-for-couples-seoul.html` | 392 | 1 |
| `th/best-area-for-shopping-seoul.html` | 487 | 1 |
| `zh-tw/accommodation.html` | 533 | 1 |
| `zh-tw/best-area-for-couples-seoul.html` | 391 | 1 |
| `zh-tw/best-area-for-shopping-seoul.html` | 486 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:533` | Seongsu | Seongsu cafe alley in Seoul | None |
| `best-area-for-couples-seoul.html:391` | Seongsu | Street in Seongsu, Seoul | None |
| `best-area-for-shopping-seoul.html:486` | Seongsu | Seongsu cafe alley in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-081 — seongsu-seoul-forest.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `seongsu-seoul-forest.png` |
| Extension | .png |
| Old Path | `images/Accommodation/seongsu-seoul-forest.png` |
| New / Current Path | `images/Accommodation/seongsu-seoul-forest.png` |
| SHA-256 | `16fd5b98cf90b2a861ef0ff0d4d9575c471014383faaab3d40832caf81865f19` |
| Dimensions | 1536 × 1024 |
| File Size | 3488566 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: park walking path with visible Seoul Forest signage; currently unreferenced. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-082 — seongsu-seoul-forest.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `seongsu-seoul-forest.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/seongsu-seoul-forest.webp` |
| New / Current Path | `images/Accommodation/seongsu-seoul-forest.webp` |
| SHA-256 | `b06ea59121dbfd09a7f756d0f1cf2687dd823c2017aa255b0733ac202e2c1443` |
| Dimensions | 1536 × 1024 |
| File Size | 1063602 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: same park walking path and Seoul Forest signage as PNG; currently unreferenced. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-083 — seoul-station-city-view.jpg

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `seoul-station-city-view.jpg` |
| Extension | .jpg |
| Old Path | `images/Accommodation/seoul-station-city-view.jpg` |
| New / Current Path | `images/Accommodation/seoul-station-city-view.jpg` |
| SHA-256 | `d607cf37011fd645e5fda41c7817dff729db15946c3cc3e539201056463b529f` |
| Dimensions | 6000 × 4000 |
| File Size | 3749823 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 20 |
| Runtime Reference Count | 20 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-first-time-visitors-seoul.html` | 505 | 1 |
| `best-area-for-solo-travelers-seoul.html` | 377 | 1 |
| `de/best-area-for-first-time-visitors-seoul.html` | 505 | 1 |
| `de/best-area-for-solo-travelers-seoul.html` | 378 | 1 |
| `de/hotels-near-seoul-station.html` | 478 | 1 |
| `es/best-area-for-first-time-visitors-seoul.html` | 365 | 1 |
| `es/best-area-for-solo-travelers-seoul.html` | 285 | 1 |
| `es/hotels-near-seoul-station.html` | 478 | 1 |
| `fr/best-area-for-first-time-visitors-seoul.html` | 505 | 1 |
| `fr/best-area-for-solo-travelers-seoul.html` | 377 | 1 |
| `fr/hotels-near-seoul-station.html` | 478 | 1 |
| `hotels-near-seoul-station.html` | 478 | 1 |
| `ja/best-area-for-first-time-visitors-seoul.html` | 505 | 1 |
| `ja/best-area-for-solo-travelers-seoul.html` | 377 | 1 |
| `ja/hotels-near-seoul-station.html` | 478 | 1 |
| `th/best-area-for-first-time-visitors-seoul.html` | 506 | 1 |
| `th/best-area-for-solo-travelers-seoul.html` | 378 | 1 |
| `zh-tw/best-area-for-first-time-visitors-seoul.html` | 505 | 1 |
| `zh-tw/best-area-for-solo-travelers-seoul.html` | 377 | 1 |
| `zh-tw/hotels-near-seoul-station.html` | 478 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-first-time-visitors-seoul.html:505` | Seoul Station | Seoul Station and surrounding city streets | Photo: Korea Tourism Organization / An Yeong-gwan |
| `best-area-for-solo-travelers-seoul.html:377` | Seoul Station | Seoul Station and surrounding cityscape in Seoul | Photo: Korea Tourism Organization / An Yeong-gwan |
| `hotels-near-seoul-station.html:478` |  | Seoul Station and surrounding city streets | Photo: Korea Tourism Organization / An Yeong-gwan |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-084 — seoul-station.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `seoul-station.png` |
| Extension | .png |
| Old Path | `images/Accommodation/seoul-station.png` |
| New / Current Path | `images/Accommodation/seoul-station.png` |
| SHA-256 | `885fea866ecfb3801ae1b902e946173d65730a747ed88f6f9d5a31d52205c953` |
| Dimensions | 1536 × 1024 |
| File Size | 2972389 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: Seoul Station exterior, transport plaza and public street. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-085 — seoul-station.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `seoul-station.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/seoul-station.webp` |
| New / Current Path | `images/Accommodation/seoul-station.webp` |
| SHA-256 | `801e43cbf54a3f1368756f26701c2dc82a6b085d739b6a61650409e6cf5453fb` |
| Dimensions | 1536 × 1024 |
| File Size | 538914 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 26 |
| Runtime Reference Count | 26 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `accommodation.html` | 482 | 1 |
| `best-area-for-families-seoul.html` | 569 | 1 |
| `best-area-for-luxury-hotels-seoul.html` | 435 | 1 |
| `best-area-for-nightlife-seoul.html` | 450 | 1 |
| `de/accommodation.html` | 482 | 1 |
| `de/best-area-for-families-seoul.html` | 569 | 1 |
| `de/best-area-for-luxury-hotels-seoul.html` | 435 | 1 |
| `de/best-area-for-nightlife-seoul.html` | 450 | 1 |
| `es/accommodation.html` | 374 | 1 |
| `es/best-area-for-families-seoul.html` | 437 | 1 |
| `es/best-area-for-luxury-hotels-seoul.html` | 333 | 1 |
| `es/best-area-for-nightlife-seoul.html` | 342 | 1 |
| `fr/accommodation.html` | 482 | 1 |
| `fr/best-area-for-families-seoul.html` | 569 | 1 |
| `fr/best-area-for-luxury-hotels-seoul.html` | 435 | 1 |
| `fr/best-area-for-nightlife-seoul.html` | 450 | 1 |
| `ja/accommodation.html` | 482 | 1 |
| `ja/best-area-for-families-seoul.html` | 569 | 1 |
| `ja/best-area-for-luxury-hotels-seoul.html` | 425 | 1 |
| `ja/best-area-for-nightlife-seoul.html` | 450 | 1 |
| `th/accommodation.html` | 483 | 1 |
| `th/best-area-for-families-seoul.html` | 570 | 1 |
| `zh-tw/accommodation.html` | 482 | 1 |
| `zh-tw/best-area-for-families-seoul.html` | 569 | 1 |
| `zh-tw/best-area-for-luxury-hotels-seoul.html` | 435 | 1 |
| `zh-tw/best-area-for-nightlife-seoul.html` | 450 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `accommodation.html:482` | Seoul Station | Seoul Station transport hub | None |
| `best-area-for-families-seoul.html:569` | Seoul Station | Seoul Station transport hub | None |
| `best-area-for-luxury-hotels-seoul.html:435` | Seoul Station / Namdaemun | Seoul Station exterior | None |
| `best-area-for-nightlife-seoul.html:450` | Seoul Station | Seoul Station transport hub | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-086 — sinchon-station-night.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `sinchon-station-night.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/sinchon-station-night.webp` |
| New / Current Path | `images/Accommodation/sinchon-station-night.webp` |
| SHA-256 | `c58e041c6fc9450f88f02d2c4e41271299b009e7c86446d716bb767b220555f7` |
| Dimensions | 1374 × 768 |
| File Size | 425306 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English img alt and enclosing area section identify a region/street/station/cultural/park scene, not a named hotel photo. |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `best-area-for-budget-travelers-seoul.html` | 466 | 1 |
| `de/best-area-for-budget-travelers-seoul.html` | 466 | 1 |
| `es/best-area-for-budget-travelers-seoul.html` | 340 | 1 |
| `fr/best-area-for-budget-travelers-seoul.html` | 466 | 1 |
| `ja/best-area-for-budget-travelers-seoul.html` | 466 | 1 |
| `th/best-area-for-budget-travelers-seoul.html` | 467 | 1 |
| `zh-tw/best-area-for-budget-travelers-seoul.html` | 466 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `best-area-for-budget-travelers-seoul.html:466` | Sinchon | Sinchon Station shopping street at night in Seoul | None |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-087 — sinsa-garosu-gil.png

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `sinsa-garosu-gil.png` |
| Extension | .png |
| Old Path | `images/Accommodation/sinsa-garosu-gil.png` |
| New / Current Path | `images/Accommodation/sinsa-garosu-gil.png` |
| SHA-256 | `b4c689b5e081bfc723100416602ecf5ae3f2f0cf3a7bfaed46bca2d0d63f1026` |
| Dimensions | 792 × 448 |
| File Size | 843012 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: tree-lined public shopping street with shops, cars and pedestrians; currently unreferenced. Exact location is not independently verified. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-088 — sinsa-garosu-gil.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | N/A — not a named hotel photograph |
| Hotel Slug | N/A |
| Filename | `sinsa-garosu-gil.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/sinsa-garosu-gil.webp` |
| New / Current Path | `images/Accommodation/sinsa-garosu-gil.webp` |
| SHA-256 | `aca721fdc96f5133093eb02c55e52cb7eb6b8819045213d21f80013fa1a608e3` |
| Dimensions | 792 × 448 |
| File Size | 125674 bytes |
| Classification | AREA |
| Hotel-specific | NO |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | AREA_NOT_MOVED |
| HTML Referencing Pages | 0 |
| Runtime Reference Count | 0 |
| Attribution Basis | Current runtime references absent; visual inspection identifies an area/street/park/cultural scene, not a named hotel photo. Visually inspected: same tree-lined public shopping street as PNG; currently unreferenced. Exact location is not independently verified. |

Runtime referring files and exact pre-move lines (all occurrences):

None. Current runtime reference count = 0.

Existing English img contexts / alt / caption (unchanged):

No current root English img reference. No alt or caption associated with a current HTML img. Classification relies on the evidence above.

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

### ASSET-089 — stay-here-again-exterior.webp

| Field | Value |
| --- | --- |
| Canonical Hotel Name | Stay Here, Again |
| Hotel Slug | stay-here-again |
| Filename | `stay-here-again-exterior.webp` |
| Extension | .webp |
| Old Path | `images/Accommodation/stay-here-again-exterior.webp` |
| New / Current Path | `images/Accommodation/hotels/stay-here-again/stay-here-again-exterior.webp` |
| SHA-256 | `1413f4e39d4025e5dd997273382504179e4055bbdfd630cad0b0cda62e62a354` |
| Dimensions | 2133 × 1145 |
| File Size | 1070252 bytes |
| Classification | HOTEL_CONFIRMED |
| Hotel-specific | YES |
| Shared common UI / multi-property | NO |
| Infographic / map | NO |
| Confidence | HIGH |
| Status | CONFIRMED_MOVED |
| HTML Referencing Pages | 7 |
| Runtime Reference Count | 7 |
| Attribution Basis | Current English where-to-stay-in-hongdae.html:380; Staying in Hongdae with 5–6 people → Stay Here, Again; img alt explicitly identifies property. All 7 language siblings use the same asset/card. |
| Reference closure | OLD runtime refs 0; NEW runtime refs 7 = BEFORE 7 |

Runtime referring files and exact pre-move lines (all occurrences):

| File | Lines | Occurrence count |
| --- | --- | --- |
| `de/where-to-stay-in-hongdae.html` | 380 | 1 |
| `es/where-to-stay-in-hongdae.html` | 380 | 1 |
| `fr/where-to-stay-in-hongdae.html` | 380 | 1 |
| `ja/where-to-stay-in-hongdae.html` | 380 | 1 |
| `th/where-to-stay-in-hongdae.html` | 381 | 1 |
| `where-to-stay-in-hongdae.html` | 380 | 1 |
| `zh-tw/where-to-stay-in-hongdae.html` | 380 | 1 |

Existing English img contexts / alt / caption (unchanged):

| English page / line | Section or card heading | Alt | Caption |
| --- | --- | --- | --- |
| `where-to-stay-in-hongdae.html:380` | Stay Here, Again | Exterior of Withus Building where Stay Here, Again is located | Photo: Kakao Map road view |

Historical reference (existing Markdown left unchanged):

None found for the exact old path in existing tracked Markdown.

## Pre-move / post-move reference manifest — moved files

| Old Path | New Path | Before runtime refs | After new refs | Old remaining | Exact referring files |
| --- | --- | --- | --- | --- | --- |
| `images/Accommodation/9-brick-hotel-exterior.png` | `images/Accommodation/hotels/9-brick-hotel/9-brick-hotel-exterior.png` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/9-brick-hotel-room.jpg` | `images/Accommodation/hotels/9-brick-hotel/9-brick-hotel-room.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/amanti-hotel-seoul-exterior.jpg` | `images/Accommodation/hotels/amanti-hotel-seoul/amanti-hotel-seoul-exterior.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/amanti-hotel-seoul-room.jpg` | `images/Accommodation/hotels/amanti-hotel-seoul/amanti-hotel-seoul-room.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/holiday-inn-express-seoul-hongdae-exterior.jpg` | `images/Accommodation/hotels/holiday-inn-express-seoul-hongdae/holiday-inn-express-seoul-hongdae-exterior.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/holiday-inn-express-seoul-hongdae-room.jpg` | `images/Accommodation/hotels/holiday-inn-express-seoul-hongdae/holiday-inn-express-seoul-hongdae-room.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/jsm-studio-hongdae-exterior.webp` | `images/Accommodation/hotels/jsm-studio-hongdae/jsm-studio-hongdae-exterior.webp` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/junibino-hotel-hongdae-exterior.jpg` | `images/Accommodation/hotels/junibino-hotel-hongdae/junibino-hotel-hongdae-exterior.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/l7-hongdae-by-lotte-hotels-exterior.webp` | `images/Accommodation/hotels/l7-hongdae/l7-hongdae-by-lotte-hotels-exterior.webp` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/l7-hongdae-by-lotte-hotels-room.jpg` | `images/Accommodation/hotels/l7-hongdae/l7-hongdae-by-lotte-hotels-room.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/mercure-ambassador-seoul-hongdae-exterior.jpg` | `images/Accommodation/hotels/mercure-ambassador-seoul-hongdae/mercure-ambassador-seoul-hongdae-exterior.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/mercure-ambassador-seoul-hongdae-room.jpg` | `images/Accommodation/hotels/mercure-ambassador-seoul-hongdae/mercure-ambassador-seoul-hongdae-room.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/ryse-autograph-collection-exterior.webp` | `images/Accommodation/hotels/ryse-autograph-collection-seoul/ryse-autograph-collection-exterior.webp` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/ryse-autograph-collection-room.jpg` | `images/Accommodation/hotels/ryse-autograph-collection-seoul/ryse-autograph-collection-room.jpg` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |
| `images/Accommodation/stay-here-again-exterior.webp` | `images/Accommodation/hotels/stay-here-again/stay-here-again-exterior.webp` | 7 | 7 | 0 | `de/where-to-stay-in-hongdae.html`<br>`es/where-to-stay-in-hongdae.html`<br>`fr/where-to-stay-in-hongdae.html`<br>`ja/where-to-stay-in-hongdae.html`<br>`th/where-to-stay-in-hongdae.html`<br>`where-to-stay-in-hongdae.html`<br>`zh-tw/where-to-stay-in-hongdae.html` |

Runtime search covers every tracked/untracked repository HTML, CSS, JS/MJS/CJS, JSON, XML and SVG file, plus TS/TSX/JSX where present. Existing protected template files were read without modification. Old/new path occurrences preserve leading / or ../ path styles. Markdown is excluded from runtime reference counts. There are no moved-photo runtime references outside HTML.

## Duplicate candidates — no deduplication

| Status | SHA-256 | Files | Action |
| --- | --- | --- | --- |
| DUPLICATE_CANDIDATE | `a1a3d931d2645aa54ec347f512b6b684da339dc68665e725a7ddfba702777215` | `images/Accommodation/gangnam-station-crossroads-kto.jpg`<br>`images/Accommodation/gangnam-station-street-family.jpg` | No move, deletion, merge or replacement. Both remain AREA assets. |

Duplicate candidates: 2 files in 1 same-SHA group. This is a secondary flag, not an extra primary classification.

## Unresolved assets

UNRESOLVED_NOT_MOVED = 0. No unresolved hotel-photo asset or conflicting multi-property attribution was found. Non-hotel classification does not independently verify all geographic provenance; the currently unreferenced Sinsa imagery is classified as public-street AREA imagery without asserting its exact location.

## Protected non-image directory files

| Path | Extension | SHA-256 | Size bytes | Status |
| --- | --- | --- | --- | --- |
| `images/Accommodation/infographic-localization-template-2026-10-02.json` | .json | `9f9af202aafc531cb7a5a5dcdb57a8f904b4cb114d2a80b01e545ac11e2231ba` | 17586 | PROTECTED_UNTRACKED — NOT_MOVED / NOT_MODIFIED / NOT_STAGED |
| `images/Accommodation/infographic-localization-template-2026-10-02.mjs` | .mjs | `53ee942070efddd5163fea72f2e57ae1cf43328bacf289e78f5f9f181e91b276` | 7344 | PROTECTED_UNTRACKED — NOT_MOVED / NOT_MODIFIED / NOT_STAGED |

These two infographic implementation templates are not images. They are included in the 91-file directory inventory but excluded from the 89-image category counts.

## Static QA and protection

| Check | Result |
| --- | --- |
| Moved assets / hotel folders | 15 / 9 |
| SHA-256 / dimensions / filenames | 15/15 unchanged; binary mismatch 0 |
| Runtime references before / after | 105 / 105 |
| Old runtime references | 0 |
| Repository HTML / public-language HTML | 367 / 364 |
| Local image references individually resolved | 2,563 |
| Srcset candidates individually parsed | 618 |
| Missing image targets / broken srcset / case mismatch | 0 / 0 / 0 |
| HTML img / picture / source / figure counts | 367/367 unchanged |
| Public text / alt / caption / affiliate / tracking drift | 0; whole-file differences restricted to the approved 15 image path replacements |
| Shared CSS / JS / JSON / XML / SVG changes | 0 |
| Existing Markdown changes | 0; approved and historical records preserved |
| Unexpected file or byte changes | 0 |
| Protected untracked4 | 4/4 same path and SHA-256; never staged |
| git diff --check / cached diff --check | PASS |
| Browser QA | Known QA Limitation: browser connection returned No browser is available. Desktop/mobile image-load, visual layout-shift and srcset selection were not observed. No retries or screenshots. Static path resolution is 100% PASS. |
| main / Production | 0 changes / deferred |


## Git packaging

- Commit 1: these 15 exact binary moves + EN/ES/JA/ZH-TW/FR/DE Hongdae runtime path changes + this Asset Map. Thai HTML excluded.
- Commit 2: `th/where-to-stay-in-hongdae.html` image path sync only.
- Push target: `th-localization-2026-10-03` only.
- No main merge/cherry-pick/push, no Production, no deployment promotion, no sitemap or reciprocal hreflang changes.
- Final commit SHAs and remote confirmation belong to the task completion report, so this map does not embed its own commit SHA.
