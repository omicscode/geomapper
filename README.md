# geomapper

 - rust geospatial mapper using dotenv for deutsch code mapper.
 - just put the files in the folder with the same naming convention.
 - Geospatial mapping data can be downloaded from here [german postcode code to location mapping](https://www.suche-postleitzahl.org/downloads).
 - Deutsch github repository[repository link](https://github.com/plzTeam).
 - polars analysis can be found here [deutsch geospatial mappers](https://github.com/zupzup/rust-polars-example).

 ```
 cargo build
 ```
 ```
 __ _    ___    ___   |  \/  |   __ _   _ __    _ __     ___   _ __
/ _` |  / _ \  / _ \  | |\/| |  / _` | | '_ \  | '_ \   / _ \ | '__|
| (_| | |  __/ | (_) | | |  | | | (_| | | |_) | | |_) | |  __/ | |
\__, |  \___|  \___/  |_|  |_|  \__,_| | .__/  | .__/   \___| |_|
|___/                                  |_|     |_|

geomapper, a geomapper for geman geographical data
 ************************************************
 Author Gaurav Sablok,
 Email: codeprog@icloud.com
 ************************************************

Usage: geomapper <COMMAND>

Commands:
plz              search according to the plz
note             search according to the note
einwohner        search according to the einwohner
qkm              search according to the qkm
latitude         search according to the latitude
longitude        search according to the longitude
osm              search according to the osm
ags              search according to the ags
ord              search according to the ort
landkreis        search according to the landkries
bundesland       search according to the bundesland
general-pattern  general pattern search
help             Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
 ```
 - search according to the plz
 ```
 gauravsablok@genome deutsch-geo-env-mapper main ?
          ./target/debug/deutsch-geo-env-mapper plz 99955
 99955   99955 Bad Tennstedt     8594    128.78479       51.15747        10.82980
 99955   2895776 16064004        Bad Tennstedt   Unstrut-Hainich-Kreis   Thüringen
 99955   2902811 16064005        Ballhausen      Unstrut-Hainich-Kreis   Thüringen
 99955   2903159 16064007        Blankenburg     Unstrut-Hainich-Kreis   Thüringen
 99955   2903160 16064009        Bruchstedt      Unstrut-Hainich-Kreis   Thüringen
 99955   2903161 16064021        Haussömmern     Unstrut-Hainich-Kreis   Thüringen
 99955   2895792 16064022        Herbsleben      Unstrut-Hainich-Kreis   Thüringen
 99955   2903162 16064027        Hornsömmern     Unstrut-Hainich-Kreis   Thüringen
 99955   2903166 16064038        Kutzleben       Unstrut-Hainich-Kreis   Thüringen
 99955   2903167 16064045        Mittelsömmern   Unstrut-Hainich-Kreis   Thüringen
 99955   2903171 16064064        Urleben Unstrut-Hainich-Kreis   Thüringen
 ```
 - search according to the note
```
./target/debug/geomapper note 01067
 01067   01067-Dresden   11957   6.866839        51.06019        13.71117
```
 - search according to the longitude
```
./target/debug/geomapper longitude 13.71117
 01067   01067 Dresden   11957   6.866839        51.06019        13.71117
```
 - search according to the latitude
```
./target/debug/geomapper latitude 51.06019
 01067   01067 Dresden   11957   6.866839        51.06019        13.71117
```
- search according to the einwohner
```
./target/debug/geomapper einwohner 11957
 01067   01067 Dresden   11957   6.866839        51.06019        13.71117
```
- search according to the osm
```
./target/debug/geomapper osm 1104550
 78267   1104550 08335001        Aach    Landkreis-Konstanz      Baden-Württemberg
 The results are: "The searched results are as follows"
```
- search according to the ags
```
 ./target/debug/geomapper ags 08335001
 78267   1104550 08335001        -A-a-c-h-       Landkreis-Konstanz      Baden-Württemberg
 The results are: "The searched results are as follows"
```
- search according to the ord
```
   ./target/debug/geomapper ord Aach
 78267   1104550 08335001        "Aach"  "Landkreis Konstanz"    "Baden-Württemberg"
 54298   1255910 07235001        "Aach"  "Landkreis Trier-Saarburg"      "Rheinland-Pfalz"
 The results are: "The searched results are as follows"
```
- search according to the landkreis. if your landkries has spaces in the name then use the general pattern search
```
./target/debug/geomapper landkreis Neckar-Odenwald-Kreis
74740   403733  08225001        "Adelsheim"   "Neckar-Odenwald-Kreis"  "Baden-Württemberg"
74858   403747  08225002        "Aglasterhausen"       "Neckar-Odenwald-Kreis" "Baden-Württemberg"
74842   403741  08225009        "Billigheim"  "Neckar-Odenwald-Kreis"  "Baden-Württemberg"
```
- search according to the bundesland. if your bundesland has spaces in the name then use the general pattern search
```
./target/debug/geomapper bundesland Mecklenburg-Vorpommern
18211   391131  13072001        "Admannshagen-Bargeshagen"      "Landkreis Rostock"     "Mecklenburg-Vorpommern"
17375   1408072 13075001        "Ahlbeck"       "Landkreis Vorpommern-Greifswald"       "Mecklenburg-Vorpommern"
18320   374204  13073001        "Ahrenshagen-Daskow"    "Landkreis Vorpommern-Rügen"    "Mecklenburg-Vorpommern"
18347   374244  13073002        "Ahrenshoop"    "Landkreis Vorpommern-Rügen"    "Mecklenburg-Vorpommern"
```

- search according to the general pattern. This means that you have spaces in your name or anything.
```
   ./target/debug/deutsch-geo-env-mapper general-pattern Konstanz
 "2784842 08335043 Konstanz 78462 Landkreis Konstanz Baden-Württemberg"
 "2784842 08335043 Konstanz 78464 Landkreis Konstanz Baden-Württemberg"
 "2784842 08335043 Konstanz 78465 Landkreis Konstanz Baden-Württemberg
 ```
Gaurav Sablok \
codeprog@icloud.com
