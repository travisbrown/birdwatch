# Community Notes

This repository contains metadata related to X's Community Notes program (formerly "Birdwatch").

## Motivation

X publishes "daily" snapshots of Community Notes data, but these snapshots do not include two useful pieces of information:

1. **User aliases:** The snapshots only show each user's "participant ID", which is a 64-character hexadecimal string, but not the three-word [alias](https://communitynotes.x.com/guide/en/contributing/aliases) that can be used to construct a link to a user's Community Notes profile. As far as I know it is not possible to link directly to a note via its note ID, or to a user via their participant ID, so the absence of aliases makes it difficult to check the current status of notes in the snapshot.
2. **Accounts posting noted tweets:** The snapshots indicate the tweet ID for each note, but not the ID (or screen name, etc.) of the account that posted the tweet. This makes it difficult to evaluate, for example, how many notes targeting Elon Musk's account are considered helpful and shown on the site (a little less than two thirds of one percent).

This repository attempts to provide as much of this missing information as possible.

## Format

All data files in this repository are in the CSV ("comma-separated values") format and can be opened in any spreadsheet application.

### Aliases

The first file schema is used by the `data/aliases.csv` file, and maps participant IDs to aliases.

```bash
$ head -n 5 data/aliases.csv
000045A5FA0CF004F68CBF2913506C37D540CF48522D33BFBF036AAC53FBDA8B,thoughtful-mint-duck
0000AE9A69E1B5D132C053E253DC42A007EDE2F11C39CF89008F447F1ABF936F,effective-juniper-bluejay
00012143F323489C81213B9D34C17FE6159338BC4CB48EB22C3FAD7742EF8144,congenial-banana-spoonbill
00018DBB934257251EBCEE91D0722C71B7DD592A57139831B4CC1BEFB0EE614E,hearty-samba-goose
0002188E5ED3028646C97CBE9ADCD12CB5B8BFAF8819BD02233645DC656EF1AD,mindful-evergreen-owl
```

This file has no column headers.
It is sorted lexicographically by the first column, and is a one-to-one mapping (no participant ID or alias appears twice).

### Missing aliases

We provide aliases for most (currently just over 96%) of the distinct participant IDs associated with notes in the snapshots.
The remaining few percent cannot currently be resolved, presumably because the Community Notes account has been deleted.

There is a second file that maps participant IDs to note IDs that the snapshots attribute to that user.
We provide this mapping because it may be possible to resolve more aliases for these in the future (using historical data or other sources).

```bash
head -n 5 workspace/unknown-aliases.csv
00034FC970AFABC3893397D8000DC1B8AD325277271DA0FC89E10CDC0B8CDCD1,1638800616637222912
000415A1E3D1DA95BD626E1D938E4A9AFFB446D1A7D53212DF1CC5A339AAEBF9,1786014873530282321
000846F219852B16C78762ABC0059ADF32A1222A754A9ABBE6EBFDD740B00B01,1773844885692711107
0016DB32296AA9EFBE2E96406EBBEBDFD57D294DB06CC82B25A3792B5AE7577F,1711670781212500284
0016DB32296AA9EFBE2E96406EBBEBDFD57D294DB06CC82B25A3792B5AE7577F,1712097857669374428
```

This file also has no headers. It is sorted lexicographically by the first column and then by the numerical value of the second column.
Each participant ID may map to many known note IDs, but each note ID will have a unique participant ID.

### Note metadata

The final file format is used by a set of files in the `data/notes` directory.
Each file includes all known notes for a particular month, indicated by the file name.

These files do each have a header that indicates the names of the columns.

```bash
$ head -n 5 data/notes/2024-08.csv
Note ID,Created at,Alias,Tweet ID,User ID,Misleading,Helpful
1818798801303253421,1722470400536,glowing-fir-whistler,1753493581866700876,3096147528,false,
1818799051443220564,1722470460173,glowing-fir-whistler,1815514603633463654,4429003533,true,
1818799135631216731,1722470480245,futuristic-reef-thicketbird,1818460037213212964,1334962540519055363,true,true
1818799164265759179,1722470487072,terrestrial-mangrove-bluebird,1818595370618044819,1202704998888624128,true,true
```

Several of the columns may have empty values. The alias column may be empty for the reasons discussed in the previous subsection.
The tweet ID column will be empty if the note only appears in the `noteStatusHistory` snapshot file, which does not provide tweet IDs.
The user ID column (which indicates the account that posted the tweet) will be empty if we have not been able to resolve the tweet (because it has been deleted, or because the account that posted it has been made private, etc.).

The final column ("Helpful") will never be empty because of missing data, but may be empty to indicate that the note has status `NEEDS_MORE_RATINGS` in the snapshot data.
A value of `true` in that column corresponds to `CURRENTLY_RATED_HELPFUL` in the snapshot data, and `false` to `CURRENTLY_RATED_NOT_HELPFUL`.

## Limitations

This repository intentionally only contains metadata. It does not include either tweet or note content.

The metadata that is here is not complete.
Approximately 12% of tweets that have received Community Notes have not been available to us during the compilation of these files.
Additionally, as noted above, some Community Notes user accounts no longer exist.
Some of these gaps may be partially filled in the future, but providing a complete record would have required beginning this project in 2021, when Birdwatch was launched.

## Most noted accounts

|          Twitter ID | Screen name     | Helpful |     % | Not helpful |     % | Needs more ratings |     % |
| ------------------: | --------------- | ------: | ----: | ----------: | ----: | -----------------: | ----: |
|            44196397 | elonmusk        |      97 |  0.63 |        1771 | 11.52 |              13499 | 87.84 |
| 1151913018936053760 | jacksonhinklle  |     336 |  8.16 |         209 |  5.07 |               3575 | 86.77 |
|           629698642 | BGatesIsaPyscho |     906 | 22.36 |          97 |  2.39 |               3049 | 75.25 |
| 1349149096909668363 | POTUS           |      57 |  1.51 |         235 |  6.23 |               3482 | 92.26 |
|          3376321847 | iluminatibot    |     624 | 17.51 |          87 |  2.44 |               2852 | 80.04 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |          98 |  2.80 |               3402 | 97.17 |
|              939091 | JoeBiden        |      15 |  0.44 |         296 |  8.64 |               3113 | 90.92 |
|           191871143 | VigilantFox     |      32 |  1.00 |          45 |  1.41 |               3121 | 97.59 |
|          1291128494 | ShaykhSulaiman  |     253 |  7.92 |         154 |  4.82 |               2789 | 87.27 |
| 1552795969959636992 | EndWokeness     |      66 |  2.07 |         128 |  4.02 |               2987 | 93.90 |
| 1446231057259433988 | DrLoupis        |     204 |  7.26 |         113 |  4.02 |               2491 | 88.71 |
|  953378142198161409 | choquei         |     368 | 13.59 |          66 |  2.44 |               2273 | 83.97 |
| 1222773302441148416 | visegrad24      |     153 |  5.70 |         254 |  9.46 |               2278 | 84.84 |
| 1319287761048723458 | MarioNawfal     |     185 |  6.96 |          84 |  3.16 |               2390 | 89.88 |
| 1326229737551912960 | libsoftiktok    |      32 |  1.35 |          72 |  3.05 |               2260 | 95.60 |
|            16106584 | stillgray       |     281 | 12.00 |          95 |  4.06 |               1966 | 83.95 |
|            30354991 | KamalaHarris    |       8 |  0.34 |         101 |  4.33 |               2222 | 95.32 |
|          3096147528 | dom_lucre       |     277 | 12.59 |          87 |  3.95 |               1836 | 83.45 |
|            76766018 | Dexerto         |     216 | 10.99 |         161 |  8.19 |               1588 | 80.81 |
| 1366565625401909249 | WallStreetSilv  |     101 |  5.38 |          66 |  3.51 |               1711 | 91.11 |
|            80233893 | jakeshieldsajj  |      97 |  5.41 |          95 |  5.30 |               1601 | 89.29 |
|  801203059359678464 | DrEliDavid      |      33 |  1.86 |         146 |  8.23 |               1596 | 89.92 |
| 1528943647185678336 | LeadingReport   |      53 |  2.99 |          30 |  1.69 |               1692 | 95.32 |
|  805532293951606785 | MattWallace888  |     216 | 12.43 |          80 |  4.60 |               1442 | 82.97 |
| 1429400366819512323 | RadioGenoa      |      50 |  2.91 |          85 |  4.95 |               1581 | 92.13 |
|          3260357396 | thehealthb0t    |     265 | 15.66 |          22 |  1.30 |               1405 | 83.04 |
| 1374968033265864706 | TaraBull808     |      78 |  4.63 |          33 |  1.96 |               1574 | 93.41 |
| 1099579684981944320 | WallStreetApes  |      85 |  5.16 |          14 |  0.85 |               1549 | 93.99 |
|           537709549 | LauraLoomer     |     197 | 12.17 |          42 |  2.59 |               1380 | 85.24 |
|  855481986290524160 | historyinmemes  |     134 |  8.38 |         167 | 10.44 |               1299 | 81.19 |
|          2538322138 | PicturesFoIder  |     100 |  6.40 |         171 | 10.94 |               1292 | 82.66 |
| 1344356576786866176 | RepMTG          |      66 |  4.49 |          50 |  3.40 |               1354 | 92.11 |
|            91416107 | OliLondonTV     |      62 |  4.45 |          81 |  5.81 |               1250 | 89.73 |
|           312696314 | Partisangirl    |     101 |  7.25 |          60 |  4.31 |               1232 | 88.44 |
|  890061634181373952 | CollinRugg      |      44 |  3.23 |          58 |  4.25 |               1262 | 92.52 |
|           971820228 | CerfiaFR        |      60 |  4.45 |          82 |  6.09 |               1205 | 89.46 |
|            14281853 | Conservatives   |      72 |  5.45 |          42 |  3.18 |               1206 | 91.36 |
| 1486069441259397125 | harris_wins     |      16 |  1.23 |          52 |  4.00 |               1233 | 94.77 |
| 1355721251180961792 | GuntherEagleman |      41 |  3.16 |          30 |  2.31 |               1225 | 94.52 |
| 1168968080690749441 | RishiSunak      |      58 |  4.58 |          69 |  5.45 |               1138 | 89.96 |
|            49849732 | petrogustavo    |      33 |  2.62 |          37 |  2.94 |               1189 | 94.44 |
| 1389913567671975937 | cb_doge         |      33 |  2.63 |          77 |  6.13 |               1146 | 91.24 |
|           133663801 | BFMTV           |     122 |  9.72 |          74 |  5.90 |               1059 | 84.38 |
|            32867753 | silvano_trotta  |     313 | 25.00 |          13 |  1.04 |                926 | 73.96 |
|            52660746 | Israel          |       6 |  0.48 |         116 |  9.27 |               1130 | 90.26 |
|            15212187 | bennyjohnson    |      65 |  5.20 |          43 |  3.44 |               1143 | 91.37 |
|           592730371 | JackPosobiec    |      73 |  5.96 |          36 |  2.94 |               1116 | 91.10 |
|            17469289 | nypost          |      92 |  7.58 |          52 |  4.29 |               1069 | 88.13 |
|            25073877 | realDonaldTrump |       5 |  0.42 |          95 |  7.92 |               1100 | 91.67 |
|  855483824351924224 | liz_churchill10 |     132 | 11.14 |          23 |  1.94 |               1030 | 86.92 |
|            37501849 | Quakeprediction |      29 |  2.51 |           1 |  0.09 |               1124 | 97.40 |
|  750683331260321792 | MAstronomers    |      68 |  5.93 |          61 |  5.32 |               1018 | 88.75 |
|           452540168 | DailyLoud       |     293 | 25.79 |          71 |  6.25 |                772 | 67.96 |
|  807355480188141568 | DramaAlert      |     189 | 16.76 |          67 |  5.94 |                872 | 77.30 |
|           133938408 | krassenstein    |      10 |  0.89 |          73 |  6.48 |               1043 | 92.63 |
| 1455903807389458436 | BladeoftheS     |     156 | 14.18 |          17 |  1.55 |                927 | 84.27 |
|           255471924 | mfa_russia      |     133 | 12.14 |          53 |  4.84 |                910 | 83.03 |
|           177101260 | Rainmaker1973   |      17 |  1.56 |         118 | 10.84 |                954 | 87.60 |
|            96684891 | kharaguchi      |     114 | 10.63 |           9 |  0.84 |                949 | 88.53 |
| 1335132884278108161 | stats_feed      |     108 | 10.11 |          55 |  5.15 |                905 | 84.74 |
|           132339474 | EdKrassen       |      20 |  1.97 |          46 |  4.54 |                947 | 93.48 |
|             7587032 | SkyNews         |      30 |  3.00 |          35 |  3.50 |                936 | 93.51 |
| 1121807798826930177 | MyLordBebo      |     107 | 10.74 |          30 |  3.01 |                859 | 86.24 |
|            69156861 | TheChiefNerd    |       5 |  0.51 |           9 |  0.92 |                964 | 98.57 |
|           428454304 | harryjsisson    |      13 |  1.33 |          31 |  3.18 |                932 | 95.49 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.48 |          23 |  2.38 |                891 | 92.14 |
|            16635277 | Acyn            |      33 |  3.42 |          64 |  6.63 |                869 | 89.96 |
| 1224185690713460736 | goddeketal      |      49 |  5.08 |          24 |  2.49 |                892 | 92.44 |
|           524869533 | QudsNen         |      58 |  6.02 |          31 |  3.22 |                875 | 90.77 |
|            90954365 | earthquakejapan |      87 |  9.09 |           0 |  0.00 |                870 | 90.91 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.45 |          35 |  3.66 |                889 | 92.89 |
| 1486473049146904576 | InternetH0F     |     101 | 10.55 |         115 | 12.02 |                741 | 77.43 |
|          2670726740 | LulaOficial     |       5 |  0.53 |          22 |  2.32 |                920 | 97.15 |
|           706646642 | EmbassyofRussia |     106 | 11.23 |          33 |  3.50 |                805 | 85.28 |
|            51241574 | AP              |      12 |  1.29 |          44 |  4.74 |                873 | 93.97 |
| 1471414739880189955 | kirawontmiss    |      45 |  4.85 |         145 | 15.62 |                738 | 79.53 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|           848279118 | Kahlissee       |      37 |  4.01 |          39 |  4.23 |                846 | 91.76 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          87 |  9.46 |                832 | 90.43 |
|            37491797 | stkirsch        |      62 |  6.75 |          11 |  1.20 |                846 | 92.06 |
|           138203134 | AOC             |      11 |  1.20 |          44 |  4.81 |                859 | 93.98 |
| 1529763962094596097 | wideawake_media |      13 |  1.48 |          13 |  1.48 |                850 | 97.03 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          23 |  2.64 |                849 | 97.36 |
| 1430497892314218502 | Resist_05       |      44 |  5.10 |          24 |  2.78 |                795 | 92.12 |
| 1043185714437992449 | catturd2        |      33 |  3.83 |          41 |  4.76 |                788 | 91.42 |
|           130557513 | mehdirhasan     |      14 |  1.64 |          26 |  3.04 |                816 | 95.33 |
|  896550698543874049 | UTDTrey         |      72 |  8.44 |          62 |  7.27 |                719 | 84.29 |
| 1538230472299270144 | ayeejuju        |      73 |  8.59 |          94 | 11.06 |                683 | 80.35 |
|             7996082 | el_pais         |      36 |  4.27 |          27 |  3.20 |                781 | 92.54 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|            11347122 | GavinNewsom     |      10 |  1.19 |          22 |  2.62 |                807 | 96.19 |
|           374712154 | TRobinsonNewEra |      38 |  4.56 |          21 |  2.52 |                775 | 92.93 |
|           292929271 | charliekirk11   |      38 |  4.59 |          24 |  2.90 |                765 | 92.50 |
| 1288319695658135552 | realstewpeters  |     179 | 21.67 |          29 |  3.51 |                618 | 74.82 |
|  875856268056969216 | DC_Draino       |      11 |  1.35 |          17 |  2.09 |                787 | 96.56 |
|            14128609 | felipeneto      |      18 |  2.21 |          28 |  3.44 |                768 | 94.35 |
|          2828212668 | AMAZlNGNATURE   |      54 |  6.67 |          98 | 12.10 |                658 | 81.23 |
| 1342174584594960384 | iamyesyouareno  |      50 |  6.17 |          47 |  5.80 |                713 | 88.02 |
| 1128337957289697281 | SprinterFamily  |     202 | 25.03 |          19 |  2.35 |                586 | 72.61 |
| 1356434353623093249 | greg16676935420 |      19 |  2.37 |          92 | 11.47 |                691 | 86.16 |
|           978932870 | CapitanBitcoin  |      36 |  4.51 |          21 |  2.63 |                742 | 92.87 |
|          3331429624 | Metropoles      |      64 |  8.05 |          21 |  2.64 |                710 | 89.31 |
|            62957739 | eduardomenoni   |     168 | 21.21 |           8 |  1.01 |                616 | 77.78 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          15 |  1.90 |                774 | 98.10 |
| 1302674142630760450 | EverythingOOC   |      66 |  8.39 |          87 | 11.05 |                634 | 80.56 |
| 1138458175663988738 | PopBase         |      46 |  5.86 |          58 |  7.39 |                681 | 86.75 |
|           109398997 | stopvaccinating |      90 | 11.51 |          10 |  1.28 |                682 | 87.21 |
|            14594813 | folha           |      28 |  3.59 |          21 |  2.69 |                732 | 93.73 |
| 1225234593789423616 | Megatron_ron    |     105 | 13.62 |          35 |  4.54 |                631 | 81.84 |
|           333357345 | Cobratate       |      28 |  3.64 |          77 | 10.00 |                665 | 86.36 |
| 1187524450809536513 | vitoquiles      |      38 |  4.95 |          22 |  2.86 |                708 | 92.19 |
| 1562038858988064768 | TheFigen\_      |      93 | 12.16 |          75 |  9.80 |                597 | 78.04 |
|  780460754910732288 | DiscussingFilm  |      29 |  3.84 |          80 | 10.58 |                647 | 85.58 |
|  855300206086168576 | HumansNoContext |      84 | 11.11 |          67 |  8.86 |                605 | 80.03 |
| 1593929531148144645 | stairwayto3dom  |      49 |  6.48 |          27 |  3.57 |                680 | 89.95 |
|          1640929196 | mmpadellan      |      22 |  2.93 |          15 |  1.99 |                715 | 95.08 |
|          1542228578 | JDVance         |       6 |  0.80 |          45 |  6.00 |                699 | 93.20 |
|          1446465174 | akafacehots     |      76 | 10.15 |          23 |  3.07 |                650 | 86.78 |
|            80820758 | JLMelenchon     |      50 |  6.70 |          45 |  6.03 |                651 | 87.27 |
| 1492007194388279333 | LibertyCappy    |      26 |  3.49 |          52 |  6.99 |                666 | 89.52 |
| 1087757588622651397 | porqueTTarg     |     150 | 20.38 |          27 |  3.67 |                559 | 75.95 |
|            46302096 | JoeyMannarinoUS |      44 |  6.04 |          20 |  2.74 |                665 | 91.22 |
|  826065164504006657 | mtgreenee       |      31 |  4.31 |          30 |  4.17 |                658 | 91.52 |
| 1080188052365029376 | acnewsitics     |       7 |  0.98 |          43 |  6.05 |                661 | 92.97 |
|            18576537 | IDF             |       0 |  0.00 |          71 | 10.00 |                639 | 90.00 |
|            27000730 | Timcast         |      13 |  1.85 |          34 |  4.83 |                657 | 93.32 |
| 1090084079964348419 | MrSinha\_       |      16 |  2.28 |           7 |  1.00 |                680 | 96.73 |
|           463142998 | SuppressedNws   |      10 |  1.43 |          31 |  4.42 |                660 | 94.15 |
| 1062754443798532096 | MakisMD         |      20 |  2.86 |          10 |  1.43 |                669 | 95.71 |
|           319774010 | kirinjisinken   |     383 | 55.03 |          11 |  1.58 |                302 | 43.39 |
| 1604139215406727170 | CensoredMen     |      34 |  4.90 |          46 |  6.63 |                614 | 88.47 |
|          1626294277 | spectatorindex  |       8 |  1.15 |          42 |  6.06 |                643 | 92.78 |
| 1138842105856573445 | NoContextHumans |      58 |  8.42 |          79 | 11.47 |                552 | 80.12 |
|          4429003533 | PopCrave        |      39 |  5.67 |          52 |  7.56 |                597 | 86.77 |
|             4239551 | amuse           |      18 |  2.62 |          21 |  3.06 |                648 | 94.32 |
|  707231479047315456 | PeterSweden7    |      21 |  3.07 |           7 |  1.02 |                655 | 95.90 |
|          4020276615 | JMilei          |       4 |  0.59 |          38 |  5.59 |                638 | 93.82 |
| 1179892477714718721 | gunsnrosesgirl3 |      21 |  3.11 |          80 | 11.83 |                575 | 85.06 |
|           330262748 | FabrizioRomano  |      52 |  7.78 |          80 | 11.98 |                536 | 80.24 |
|          1500129642 | MattWalshBlog   |      10 |  1.50 |          27 |  4.05 |                630 | 94.45 |
|  959531564341317632 | AlertesInfos    |      29 |  4.35 |          40 |  6.00 |                598 | 89.66 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.92 |          34 |  5.13 |                603 | 90.95 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.51 |          25 |  3.77 |                628 | 94.72 |
|           535707261 | eldiarioes      |       8 |  1.21 |          13 |  1.96 |                641 | 96.83 |
|          2161051908 | AvivaKlompas    |      13 |  1.97 |          60 |  9.10 |                586 | 88.92 |
|             1137701 | DavidSacks      |      20 |  3.04 |          48 |  7.31 |                589 | 89.65 |
|           472412809 | f_philippot     |     103 | 15.90 |          13 |  2.01 |                532 | 82.10 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
|           294293982 | Rothmus         |      39 |  6.04 |          34 |  5.26 |                573 | 88.70 |
|            14436030 | elmundoes       |      57 |  8.88 |          12 |  1.87 |                573 | 89.25 |
|           337808606 | RobertKennedyJr |      11 |  1.72 |          30 |  4.68 |                600 | 93.60 |
| 1625843518643085312 | creepydotorg    |      53 |  8.28 |          46 |  7.19 |                541 | 84.53 |
| 1111976778065723393 | nocontextfooty  |      63 |  9.95 |          53 |  8.37 |                517 | 81.67 |
|            19069018 | jreichelt       |      17 |  2.70 |          18 |  2.86 |                594 | 94.44 |
|              759251 | CNN             |      16 |  2.56 |          15 |  2.40 |                595 | 95.05 |
| 1392864463204782082 | WarMonitors     |      17 |  2.75 |          28 |  4.52 |                574 | 92.73 |
| 1155845777039810560 | richimedhurst   |      17 |  2.75 |          22 |  3.56 |                579 | 93.69 |
|            14260960 | JustinTrudeau   |       1 |  0.16 |          45 |  7.29 |                571 | 92.54 |
|           288277167 | atrupar         |      15 |  2.44 |          22 |  3.58 |                577 | 93.97 |
| 1434450096557596680 | PolitlcsUK      |      16 |  2.61 |          41 |  6.68 |                557 | 90.72 |
| 1661674273122160641 | \_maakun\_\_2   |     120 | 19.61 |           0 |  0.00 |                492 | 80.39 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.61 |          16 |  2.63 |                564 | 92.76 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      51 |  8.40 |           5 |  0.82 |                551 | 90.77 |
|            55329156 | RNCResearch     |      31 |  5.12 |          19 |  3.14 |                556 | 91.75 |
| 1158115510606815232 | therealbuni     |      65 | 10.73 |          22 |  3.63 |                519 | 85.64 |
| 1227799690579718144 | VivekGRamaswamy |      13 |  2.15 |          30 |  4.95 |                563 | 92.90 |
|            19923515 | abc_es          |      69 | 11.44 |          18 |  2.99 |                516 | 85.57 |
|            62513246 | jk_rowling      |       0 |  0.00 |          50 |  8.31 |                552 | 91.69 |
|            19017675 | Nigel_Farage    |       5 |  0.83 |          22 |  3.66 |                574 | 95.51 |
|  795188519115358208 | tweetsoku1      |     198 | 33.06 |           4 |  0.67 |                397 | 66.28 |
|  918197046871502849 | siteptbr        |      70 | 11.73 |          17 |  2.85 |                510 | 85.43 |
|          4691437897 | darrengrimes\_  |      31 |  5.24 |          15 |  2.53 |                546 | 92.23 |
| 1298372735383605249 | RonFilipkowski  |      16 |  2.71 |          30 |  5.08 |                545 | 92.22 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.38 |                574 | 97.62 |
|            14173315 | NBCNews         |      11 |  1.87 |          17 |  2.90 |                559 | 95.23 |
|           195853497 | EuropeInvasions |      33 |  5.65 |          37 |  6.34 |                514 | 88.01 |
|           458008892 | EylonALevy      |       0 |  0.00 |          38 |  6.51 |                546 | 93.49 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.37 |          60 | 10.27 |                516 | 88.36 |
| 1483750637275860993 | catsscareme2021 |      21 |  3.61 |          11 |  1.89 |                550 | 94.50 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.74 |          16 |  2.78 |                550 | 95.49 |
|            14377605 | TheDemocrats    |       2 |  0.35 |          20 |  3.50 |                550 | 96.15 |
| 1450241520859156483 | geoscience16    |     159 | 27.85 |           4 |  0.70 |                408 | 71.45 |
| 1200616796295847936 | unusual_whales  |      73 | 12.83 |          21 |  3.69 |                475 | 83.48 |
| 1201670995435646976 | laurenboebert   |      17 |  3.00 |          22 |  3.88 |                528 | 93.12 |
| 1432287556129812484 | himuro398       |     113 | 20.04 |           7 |  1.24 |                444 | 78.72 |
|              742143 | BBCWorld        |      21 |  3.73 |          32 |  5.68 |                510 | 90.59 |
| 1339166129110065152 | GBNEWS          |      24 |  4.31 |           7 |  1.26 |                526 | 94.43 |
|           611986351 | KimDotcom       |      27 |  4.86 |          32 |  5.76 |                497 | 89.39 |
|            81371986 | LozzaFox        |      29 |  5.23 |          18 |  3.24 |                508 | 91.53 |
|            94324983 | FonsiLoaiza     |      10 |  1.82 |          17 |  3.09 |                523 | 95.09 |
|             5734902 | tagesschau      |      14 |  2.55 |          21 |  3.83 |                513 | 93.61 |
|            39692424 | AlertaNews24    |     104 | 19.22 |          16 |  2.96 |                421 | 77.82 |
| 1393726565809278976 | AdameMedia      |      12 |  2.23 |          23 |  4.27 |                504 | 93.51 |
|              807095 | nytimes         |      19 |  3.53 |          19 |  3.53 |                500 | 92.94 |
|           109065990 | RealAlexJones   |      39 |  7.26 |          13 |  2.42 |                485 | 90.32 |
|            65045121 | owenjonesjourno |       9 |  1.69 |          21 |  3.93 |                504 | 94.38 |
|           371381075 | sandrousseau    |      29 |  5.51 |          44 |  8.37 |                453 | 86.12 |
|            36749572 | Alphafox78      |      96 | 18.29 |          34 |  6.48 |                395 | 75.24 |
|  896466491587080194 | greg_price11    |      17 |  3.24 |          20 |  3.81 |                488 | 92.95 |
| 1492677599390322689 | weirddalle      |      18 |  3.46 |          64 | 12.31 |                438 | 84.23 |
