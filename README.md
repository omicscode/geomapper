# deutsch-geo-env-mapper
 - rust geospatial mapper.
 - just put the files in the folder and then just search. No time for reload multiple times. Uses dotenv for automatic file indexing. 
 - application of dotenv for geo spatial command line detusch code mapper. 
 - polars analysis can be found here [deutsch geospatial mappers](https://github.com/zupzup/rust-polars-example)
 - You can download the geospatial mapping from here [german postcode code to location mapping](https://www.suche-postleitzahl.org/downloads).
 - please see the last commit message and if it says compiled binary then it is completed or else still in development version.
 - Their github repository[repository link](https://github.com/plzTeam).

 ```
 cargo build
 ```
 ```
 gauravsablok@genome deutsch-geo-env-mapper main ? ./target/debug/deutsch-geo-env-mapper -h
 deutsch geo-mapper

 Usage: deutsch-geo-env-mapper <COMMAND>

 Commands:
  plz         search according to the plz
  note        search according to the note
  einwohner   search according to the einwohner
  qkm         search according to the qkm
  latitude    search according to the latitude
  longitude   search according to the longitude
  osm         search according to the osm
  ags         search according to the ags
  ord         search according to the ort
  landkreis   search according to the landkries
  bundesland  search according to the bundesland
  help        Print this message or the help of the given subcommand(s)

 Options:
  -h, --help     Print help
  -V, --version  Print version

 ```

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
 
 21:51:03 gauravsablok@genome deutsch-geo-env-mapper main ? 
     ./target/debug/deutsch-geo-env-mapper note 01067
 01067   01067-Dresden   11957   6.866839        51.06019        13.71117
 21:51:22 gauravsablok@genome deutsch-geo-env-mapper main ? 
     ./target/debug/deutsch-geo-env-mapper longitude 13.71117
 01067   01067 Dresden   11957   6.866839        51.06019        13.71117
 21:51:33 gauravsablok@genome deutsch-geo-env-mapper main ? 
     ./target/debug/deutsch-geo-env-mapper latitude 51.06019
 01067   01067 Dresden   11957   6.866839        51.06019        13.71117
 21:51:40 gauravsablok@genome deutsch-geo-env-mapper main ? 
     ./target/debug/deutsch-geo-env-mapper einwohner 11957
 01067   01067 Dresden   11957   6.866839        51.06019        13.71117

 ```
 Gaurav Sablok
