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
|            44196397 | elonmusk        |      97 |  0.64 |        1757 | 11.60 |              13296 | 87.76 |
| 1151913018936053760 | jacksonhinklle  |     335 |  8.16 |         205 |  5.00 |               3563 | 86.84 |
|           629698642 | BGatesIsaPyscho |     901 | 22.46 |          95 |  2.37 |               3015 | 75.17 |
| 1349149096909668363 | POTUS           |      57 |  1.51 |         234 |  6.22 |               3473 | 92.27 |
|          3376321847 | iluminatibot    |     621 | 17.55 |          87 |  2.46 |               2831 | 79.99 |
|          3315264553 | KamalaHQ        |       1 |  0.03 |          95 |  2.78 |               3325 | 97.19 |
|              939091 | JoeBiden        |      15 |  0.44 |         290 |  8.57 |               3080 | 90.99 |
|          1291128494 | ShaykhSulaiman  |     253 |  7.94 |         152 |  4.77 |               2780 | 87.28 |
|           191871143 | VigilantFox     |      32 |  1.01 |          45 |  1.42 |               3083 | 97.56 |
| 1552795969959636992 | EndWokeness     |      66 |  2.09 |         125 |  3.96 |               2962 | 93.94 |
| 1446231057259433988 | DrLoupis        |     204 |  7.29 |         111 |  3.97 |               2483 | 88.74 |
|  953378142198161409 | choquei         |     366 | 13.55 |          66 |  2.44 |               2270 | 84.01 |
| 1222773302441148416 | visegrad24      |     153 |  5.73 |         254 |  9.51 |               2265 | 84.77 |
| 1319287761048723458 | MarioNawfal     |     184 |  7.00 |          81 |  3.08 |               2363 | 89.92 |
| 1326229737551912960 | libsoftiktok    |      32 |  1.37 |          71 |  3.04 |               2235 | 95.59 |
|            16106584 | stillgray       |     279 | 11.99 |          95 |  4.08 |               1952 | 83.92 |
|            30354991 | KamalaHarris    |       8 |  0.36 |          97 |  4.31 |               2143 | 95.33 |
|          3096147528 | dom_lucre       |     277 | 12.61 |          86 |  3.92 |               1833 | 83.47 |
|            76766018 | Dexerto         |     215 | 11.00 |         159 |  8.13 |               1581 | 80.87 |
| 1366565625401909249 | WallStreetSilv  |     101 |  5.39 |          65 |  3.47 |               1708 | 91.14 |
|            80233893 | jakeshieldsajj  |      97 |  5.43 |          94 |  5.26 |               1596 | 89.31 |
|  801203059359678464 | DrEliDavid      |      33 |  1.88 |         141 |  8.02 |               1585 | 90.11 |
| 1528943647185678336 | LeadingReport   |      53 |  3.02 |          29 |  1.65 |               1672 | 95.32 |
|  805532293951606785 | MattWallace888  |     212 | 12.33 |          80 |  4.65 |               1427 | 83.01 |
| 1429400366819512323 | RadioGenoa      |      50 |  2.93 |          84 |  4.92 |               1574 | 92.15 |
|          3260357396 | thehealthb0t    |     259 | 15.54 |          22 |  1.32 |               1386 | 83.14 |
| 1374968033265864706 | TaraBull808     |      74 |  4.46 |          32 |  1.93 |               1553 | 93.61 |
| 1099579684981944320 | WallStreetApes  |      84 |  5.19 |          14 |  0.86 |               1522 | 93.95 |
|           537709549 | LauraLoomer     |     197 | 12.18 |          42 |  2.60 |               1378 | 85.22 |
|  855481986290524160 | historyinmemes  |     133 |  8.38 |         165 | 10.40 |               1289 | 81.22 |
|          2538322138 | PicturesFoIder  |      99 |  6.42 |         166 | 10.76 |               1278 | 82.83 |
| 1344356576786866176 | RepMTG          |      66 |  4.51 |          50 |  3.42 |               1348 | 92.08 |
|           312696314 | Partisangirl    |     101 |  7.29 |          60 |  4.33 |               1225 | 88.38 |
|            91416107 | OliLondonTV     |      62 |  4.49 |          80 |  5.79 |               1240 | 89.73 |
|           971820228 | CerfiaFR        |      60 |  4.51 |          82 |  6.16 |               1189 | 89.33 |
|  890061634181373952 | CollinRugg      |      44 |  3.34 |          55 |  4.17 |               1220 | 92.49 |
|            14281853 | Conservatives   |      72 |  5.48 |          40 |  3.04 |               1203 | 91.48 |
| 1486069441259397125 | harris_wins     |      16 |  1.24 |          53 |  4.11 |               1222 | 94.66 |
| 1355721251180961792 | GuntherEagleman |      39 |  3.04 |          30 |  2.34 |               1215 | 94.63 |
| 1168968080690749441 | RishiSunak      |      58 |  4.61 |          66 |  5.25 |               1133 | 90.14 |
| 1389913567671975937 | cb_doge         |      33 |  2.63 |          76 |  6.07 |               1144 | 91.30 |
|           133663801 | BFMTV           |     120 |  9.62 |          72 |  5.77 |               1056 | 84.62 |
|            52660746 | Israel          |       6 |  0.48 |         115 |  9.25 |               1122 | 90.27 |
|            49849732 | petrogustavo    |      33 |  2.66 |          35 |  2.82 |               1173 | 94.52 |
|            32867753 | silvano_trotta  |     309 | 24.98 |          12 |  0.97 |                916 | 74.05 |
|            15212187 | bennyjohnson    |      64 |  5.26 |          40 |  3.29 |               1113 | 91.45 |
|           592730371 | JackPosobiec    |      73 |  6.03 |          35 |  2.89 |               1102 | 91.07 |
|            17469289 | nypost          |      91 |  7.55 |          50 |  4.15 |               1064 | 88.30 |
|  855483824351924224 | liz_churchill10 |     132 | 11.19 |          22 |  1.86 |               1026 | 86.95 |
|            37501849 | Quakeprediction |      29 |  2.51 |           1 |  0.09 |               1124 | 97.40 |
|           452540168 | DailyLoud       |     290 | 25.69 |          71 |  6.29 |                768 | 68.02 |
|  750683331260321792 | MAstronomers    |      68 |  6.03 |          60 |  5.32 |               1000 | 88.65 |
|  807355480188141568 | DramaAlert      |     188 | 16.85 |          65 |  5.82 |                863 | 77.33 |
|            25073877 | realDonaldTrump |       5 |  0.45 |          93 |  8.35 |               1016 | 91.20 |
|           133938408 | krassenstein    |      10 |  0.90 |          71 |  6.41 |               1027 | 92.69 |
| 1455903807389458436 | BladeoftheS     |     154 | 14.03 |          16 |  1.46 |                928 | 84.52 |
|           255471924 | mfa_russia      |     133 | 12.17 |          53 |  4.85 |                907 | 82.98 |
|           177101260 | Rainmaker1973   |      17 |  1.57 |         118 | 10.93 |                945 | 87.50 |
|            96684891 | kharaguchi      |     114 | 10.68 |           9 |  0.84 |                944 | 88.47 |
| 1335132884278108161 | stats_feed      |     107 | 10.19 |          54 |  5.14 |                889 | 84.67 |
|           132339474 | EdKrassen       |      20 |  2.00 |          46 |  4.59 |                936 | 93.41 |
|             7587032 | SkyNews         |      30 |  3.02 |          34 |  3.42 |                930 | 93.56 |
| 1121807798826930177 | MyLordBebo      |     106 | 10.70 |          30 |  3.03 |                855 | 86.28 |
|            69156861 | TheChiefNerd    |       5 |  0.51 |           9 |  0.92 |                962 | 98.57 |
|           428454304 | harryjsisson    |      13 |  1.34 |          31 |  3.19 |                927 | 95.47 |
|           524869533 | QudsNen         |      58 |  6.02 |          31 |  3.22 |                875 | 90.77 |
| 1224185690713460736 | goddeketal      |      48 |  4.98 |          24 |  2.49 |                891 | 92.52 |
| 1577761560394665984 | DiedSuddenly\_  |      53 |  5.50 |          23 |  2.39 |                887 | 92.11 |
|            90954365 | earthquakejapan |      87 |  9.17 |           0 |  0.00 |                862 | 90.83 |
| 1323730225067339784 | WhiteHouse      |      33 |  3.48 |          33 |  3.48 |                883 | 93.05 |
|           706646642 | EmbassyofRussia |     106 | 11.23 |          33 |  3.50 |                805 | 85.28 |
|            16635277 | Acyn            |      33 |  3.50 |          64 |  6.79 |                846 | 89.71 |
| 1486473049146904576 | InternetH0F     |     100 | 10.60 |         111 | 11.77 |                732 | 77.62 |
|          2670726740 | LulaOficial     |       5 |  0.53 |          22 |  2.35 |                911 | 97.12 |
| 1319036828964454402 | CommunityNotes  |       0 |  0.00 |          36 |  3.90 |                888 | 96.10 |
|            51241574 | AP              |      12 |  1.30 |          44 |  4.77 |                866 | 93.93 |
|            22703645 | TuckerCarlson   |       1 |  0.11 |          86 |  9.36 |                832 | 90.53 |
|            37491797 | stkirsch        |      62 |  6.75 |          11 |  1.20 |                846 | 92.06 |
| 1471414739880189955 | kirawontmiss    |      45 |  4.92 |         142 | 15.54 |                727 | 79.54 |
|           848279118 | Kahlissee       |      37 |  4.06 |          39 |  4.28 |                836 | 91.67 |
|           138203134 | AOC             |      11 |  1.22 |          41 |  4.55 |                849 | 94.23 |
|           242827267 | PierrePoilievre |       0 |  0.00 |          23 |  2.65 |                845 | 97.35 |
| 1529763962094596097 | wideawake_media |      13 |  1.50 |          12 |  1.39 |                840 | 97.11 |
| 1430497892314218502 | Resist_05       |      45 |  5.23 |          24 |  2.79 |                792 | 91.99 |
| 1043185714437992449 | catturd2        |      33 |  3.84 |          41 |  4.77 |                785 | 91.39 |
|  896550698543874049 | UTDTrey         |      71 |  8.37 |          60 |  7.08 |                717 | 84.55 |
| 1538230472299270144 | ayeejuju        |      73 |  8.61 |          94 | 11.08 |                681 | 80.31 |
|           130557513 | mehdirhasan     |      14 |  1.65 |          26 |  3.07 |                806 | 95.27 |
| 1431774993419956224 | jhmdrei         |     228 | 27.14 |           3 |  0.36 |                609 | 72.50 |
|             7996082 | el_pais         |      36 |  4.31 |          27 |  3.23 |                773 | 92.46 |
|            11347122 | GavinNewsom     |      10 |  1.20 |          22 |  2.63 |                804 | 96.17 |
|           374712154 | TRobinsonNewEra |      38 |  4.61 |          20 |  2.42 |                767 | 92.97 |
| 1288319695658135552 | realstewpeters  |     179 | 21.75 |          29 |  3.52 |                615 | 74.73 |
|            14128609 | felipeneto      |      18 |  2.21 |          28 |  3.44 |                767 | 94.34 |
|  875856268056969216 | DC_Draino       |      11 |  1.37 |          17 |  2.11 |                776 | 96.52 |
| 1128337957289697281 | SprinterFamily  |     201 | 25.03 |          19 |  2.37 |                583 | 72.60 |
| 1356434353623093249 | greg16676935420 |      19 |  2.37 |          92 | 11.47 |                691 | 86.16 |
|           292929271 | charliekirk11   |      38 |  4.76 |          23 |  2.88 |                737 | 92.36 |
|          2828212668 | AMAZlNGNATURE   |      54 |  6.77 |          94 | 11.78 |                650 | 81.45 |
| 1342174584594960384 | iamyesyouareno  |      49 |  6.16 |          46 |  5.78 |                701 | 88.07 |
|           978932870 | CapitanBitcoin  |      36 |  4.56 |          21 |  2.66 |                732 | 92.78 |
|  935142655213703168 | zoo_bear        |       0 |  0.00 |          15 |  1.90 |                773 | 98.10 |
|          3331429624 | Metropoles      |      64 |  8.14 |          21 |  2.67 |                701 | 89.19 |
| 1302674142630760450 | EverythingOOC   |      66 |  8.43 |          87 | 11.11 |                630 | 80.46 |
|           109398997 | stopvaccinating |      90 | 11.51 |          10 |  1.28 |                682 | 87.21 |
|            62957739 | eduardomenoni   |     164 | 21.05 |           8 |  1.03 |                607 | 77.92 |
| 1138458175663988738 | PopBase         |      46 |  5.91 |          58 |  7.45 |                675 | 86.65 |
|            14594813 | folha           |      29 |  3.73 |          21 |  2.70 |                728 | 93.57 |
| 1225234593789423616 | Megatron_ron    |     105 | 13.74 |          35 |  4.58 |                624 | 81.68 |
|           333357345 | Cobratate       |      28 |  3.67 |          75 |  9.84 |                659 | 86.48 |
| 1187524450809536513 | vitoquiles      |      38 |  4.99 |          22 |  2.89 |                702 | 92.13 |
| 1562038858988064768 | TheFigen\_      |      93 | 12.27 |          74 |  9.76 |                591 | 77.97 |
| 1593929531148144645 | stairwayto3dom  |      49 |  6.49 |          27 |  3.58 |                679 | 89.93 |
|  780460754910732288 | DiscussingFilm  |      29 |  3.85 |          80 | 10.61 |                645 | 85.54 |
|          1640929196 | mmpadellan      |      22 |  2.93 |          15 |  2.00 |                714 | 95.07 |
|          1446465174 | akafacehots     |      76 | 10.20 |          23 |  3.09 |                646 | 86.71 |
| 1492007194388279333 | LibertyCappy    |      26 |  3.50 |          52 |  7.00 |                665 | 89.50 |
|            80820758 | JLMelenchon     |      49 |  6.64 |          43 |  5.83 |                646 | 87.53 |
|          1542228578 | JDVance         |       6 |  0.83 |          43 |  5.92 |                677 | 93.25 |
| 1087757588622651397 | porqueTTarg     |     146 | 20.11 |          26 |  3.58 |                554 | 76.31 |
|            46302096 | JoeyMannarinoUS |      42 |  5.87 |          19 |  2.65 |                655 | 91.48 |
|  826065164504006657 | mtgreenee       |      30 |  4.19 |          30 |  4.19 |                656 | 91.62 |
|            18576537 | IDF             |       0 |  0.00 |          71 | 10.03 |                637 | 89.97 |
|            27000730 | Timcast         |      13 |  1.85 |          34 |  4.83 |                657 | 93.32 |
| 1080188052365029376 | acnewsitics     |       7 |  1.00 |          42 |  5.98 |                653 | 93.02 |
| 1090084079964348419 | MrSinha\_       |      16 |  2.28 |           7 |  1.00 |                678 | 96.72 |
| 1062754443798532096 | MakisMD         |      20 |  2.86 |          10 |  1.43 |                669 | 95.71 |
|           463142998 | SuppressedNws   |      10 |  1.43 |          31 |  4.45 |                656 | 94.12 |
| 1604139215406727170 | CensoredMen     |      34 |  4.91 |          46 |  6.65 |                612 | 88.44 |
|          1626294277 | spectatorindex  |       8 |  1.16 |          42 |  6.10 |                639 | 92.74 |
|           319774010 | kirinjisinken   |     379 | 55.17 |          11 |  1.60 |                297 | 43.23 |
|             4239551 | amuse           |      18 |  2.63 |          21 |  3.07 |                645 | 94.30 |
|          4429003533 | PopCrave        |      39 |  5.74 |          50 |  7.36 |                590 | 86.89 |
| 1138842105856573445 | NoContextHumans |      57 |  8.39 |          77 | 11.34 |                545 | 80.27 |
|          4020276615 | JMilei          |       4 |  0.59 |          38 |  5.61 |                635 | 93.80 |
|  707231479047315456 | PeterSweden7    |      21 |  3.10 |           7 |  1.03 |                649 | 95.86 |
| 1179892477714718721 | gunsnrosesgirl3 |      21 |  3.12 |          79 | 11.76 |                572 | 85.12 |
|  855300206086168576 | HumansNoContext |      53 |  7.91 |          67 | 10.00 |                550 | 82.09 |
|  959531564341317632 | AlertesInfos    |      29 |  4.35 |          40 |  6.01 |                597 | 89.64 |
|            39344374 | DonaldJTrumpJr  |      26 |  3.94 |          34 |  5.15 |                600 | 90.91 |
|           330262748 | FabrizioRomano  |      52 |  7.88 |          79 | 11.97 |                529 | 80.15 |
| 1686901686185721857 | TrumpDailyPosts |      10 |  1.52 |          25 |  3.79 |                625 | 94.70 |
|          2161051908 | AvivaKlompas    |      13 |  1.98 |          59 |  8.98 |                585 | 89.04 |
|             1137701 | DavidSacks      |      20 |  3.05 |          48 |  7.32 |                588 | 89.63 |
|           535707261 | eldiarioes      |       8 |  1.22 |          13 |  1.98 |                635 | 96.80 |
|          1500129642 | MattWalshBlog   |      10 |  1.53 |          27 |  4.13 |                616 | 94.33 |
| 1821294404000477185 | MedicsAutoInc   |      60 |  9.26 |           0 |  0.00 |                588 | 90.74 |
|           294293982 | Rothmus         |      38 |  5.89 |          35 |  5.43 |                572 | 88.68 |
|           472412809 | f_philippot     |     103 | 15.99 |          12 |  1.86 |                529 | 82.14 |
|           337808606 | RobertKennedyJr |      11 |  1.72 |          30 |  4.69 |                598 | 93.58 |
| 1625843518643085312 | creepydotorg    |      53 |  8.31 |          46 |  7.21 |                539 | 84.48 |
|            14436030 | elmundoes       |      57 |  9.06 |          11 |  1.75 |                561 | 89.19 |
|            19069018 | jreichelt       |      17 |  2.71 |          18 |  2.87 |                593 | 94.43 |
| 1111976778065723393 | nocontextfooty  |      63 | 10.03 |          53 |  8.44 |                512 | 81.53 |
|              759251 | CNN             |      16 |  2.56 |          15 |  2.40 |                594 | 95.04 |
| 1392864463204782082 | WarMonitors     |      17 |  2.75 |          28 |  4.53 |                573 | 92.72 |
|            14260960 | JustinTrudeau   |       1 |  0.16 |          46 |  7.49 |                567 | 92.35 |
| 1155845777039810560 | richimedhurst   |      17 |  2.78 |          22 |  3.60 |                572 | 93.62 |
|            55329156 | RNCResearch     |      31 |  5.12 |          19 |  3.14 |                556 | 91.75 |
|           288277167 | atrupar         |      15 |  2.48 |          19 |  3.14 |                572 | 94.39 |
|  930212715422998530 | Travis_4_Trump  |      28 |  4.62 |          16 |  2.64 |                562 | 92.74 |
|            62513246 | jk_rowling      |       0 |  0.00 |          51 |  8.47 |                551 | 91.53 |
|            19923515 | abc_es          |      69 | 11.48 |          18 |  3.00 |                514 | 85.52 |
| 1661674273122160641 | \_maakun\_\_2   |     117 | 19.47 |           0 |  0.00 |                484 | 80.53 |
| 1227799690579718144 | VivekGRamaswamy |      13 |  2.17 |          29 |  4.83 |                558 | 93.00 |
| 1434450096557596680 | PolitlcsUK      |      16 |  2.68 |          41 |  6.86 |                541 | 90.47 |
|            19017675 | Nigel_Farage    |       5 |  0.84 |          22 |  3.71 |                566 | 95.45 |
| 1158115510606815232 | therealbuni     |      65 | 10.96 |          22 |  3.71 |                506 | 85.33 |
|  918197046871502849 | siteptbr        |      71 | 11.99 |          17 |  2.87 |                504 | 85.14 |
|  795188519115358208 | tweetsoku1      |     194 | 32.83 |           4 |  0.68 |                393 | 66.50 |
|          4691437897 | darrengrimes\_  |      31 |  5.27 |          15 |  2.55 |                542 | 92.18 |
|            16397147 | liberal_party   |       0 |  0.00 |          14 |  2.39 |                573 | 97.61 |
| 1298372735383605249 | RonFilipkowski  |      15 |  2.56 |          30 |  5.11 |                542 | 92.33 |
|            14173315 | NBCNews         |      11 |  1.88 |          17 |  2.91 |                557 | 95.21 |
| 1160201304938913797 | JINKOUZOUKA_jp  |      47 |  8.03 |           5 |  0.85 |                533 | 91.11 |
|           195853497 | EuropeInvasions |      33 |  5.66 |          37 |  6.35 |                513 | 87.99 |
| 1221462414744596483 | RpsAgainstTrump |       8 |  1.37 |          59 | 10.14 |                515 | 88.49 |
|           458008892 | EylonALevy      |       0 |  0.00 |          38 |  6.56 |                541 | 93.44 |
| 1483750637275860993 | catsscareme2021 |      21 |  3.63 |          10 |  1.73 |                547 | 94.64 |
|  818893114979061761 | JoJoFromJerz    |      10 |  1.74 |          16 |  2.79 |                548 | 95.47 |
|            14377605 | TheDemocrats    |       2 |  0.35 |          19 |  3.35 |                547 | 96.30 |
| 1201670995435646976 | laurenboebert   |      17 |  3.00 |          22 |  3.88 |                528 | 93.12 |
| 1200616796295847936 | unusual_whales  |      73 | 12.94 |          21 |  3.72 |                470 | 83.33 |
| 1450241520859156483 | geoscience16    |     160 | 28.42 |           4 |  0.71 |                399 | 70.87 |
|              742143 | BBCWorld        |      21 |  3.74 |          32 |  5.70 |                508 | 90.55 |
|           611986351 | KimDotcom       |      27 |  4.86 |          32 |  5.76 |                497 | 89.39 |
| 1432287556129812484 | himuro398       |     112 | 20.14 |           6 |  1.08 |                438 | 78.78 |
| 1339166129110065152 | GBNEWS          |      23 |  4.14 |           7 |  1.26 |                525 | 94.59 |
|            81371986 | LozzaFox        |      29 |  5.25 |          18 |  3.26 |                505 | 91.49 |
|             5734902 | tagesschau      |      14 |  2.55 |          22 |  4.01 |                512 | 93.43 |
|            39692424 | AlertaNews24    |     104 | 19.29 |          16 |  2.97 |                419 | 77.74 |
|              807095 | nytimes         |      19 |  3.54 |          19 |  3.54 |                499 | 92.92 |
|           109065990 | RealAlexJones   |      39 |  7.29 |          13 |  2.43 |                483 | 90.28 |
| 1393726565809278976 | AdameMedia      |      12 |  2.24 |          22 |  4.11 |                501 | 93.64 |
|            65045121 | owenjonesjourno |       9 |  1.69 |          21 |  3.94 |                503 | 94.37 |
|            94324983 | FonsiLoaiza     |      10 |  1.88 |          17 |  3.20 |                505 | 94.92 |
|           371381075 | sandrousseau    |      28 |  5.36 |          44 |  8.43 |                450 | 86.21 |
|  896466491587080194 | greg_price11    |      17 |  3.28 |          20 |  3.85 |                482 | 92.87 |
| 1492677599390322689 | weirddalle      |      17 |  3.28 |          64 | 12.36 |                437 | 84.36 |
| 1504902093433417730 | narrative_hole  |      26 |  5.03 |          27 |  5.22 |                464 | 89.75 |
