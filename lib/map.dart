import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:flutter_rust_app/bridge_generated.dart';
import 'package:latlong2/latlong.dart';

Widget buildMapPageUi(List<GeoMap> geoMapList) {
  return MaterialApp(
    home: Scaffold(
      appBar: AppBar(title: const Text('Flutter Rust Application Map Example')),
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          children: [
            const Padding(
              padding: EdgeInsets.only(top: 8.0, bottom: 8.0),
              child: Text(
                  'This is a map that is showing Anhui Anqing(30.506375, 117.044965).'),
            ),
            Flexible(
              child: createMapLayers(geoMapList),
            ),
          ],
        ),
      ),
    ),
  );
}

Widget createMapLayers(List<GeoMap> geoMapList) => FlutterMap(
      options: MapOptions(
        minZoom: 1,
        crs: const Epsg4326(),
        center: LatLng(30.506375, 117.044965),
        zoom: 10.0,
      ),
      layers: [
        TileLayerOptions(
          wmsOptions: WMSTileLayerOptions(
            crs: const Epsg4326(),
            baseUrl: 'https://ows.terrestris.de/osm/service?',
            layers: ['TOPO-OSM-WMS'],
          ),
          userAgentPackageName: 'dev.fleaflet.flutter_map.example',
        ),
        for (GeoMap geoMap in geoMapList)
          PolygonLayerOptions(
            polygons: [
              Polygon(
                label: geoMap.name,
                labelStyle: const TextStyle(fontWeight: FontWeight.bold),
                borderStrokeWidth: 3.0,
                borderColor: Colors.red,
                points: [
                  for (Position position in geoMap.coordinates)
                    LatLng(position.latitude, position.longitude),
                ],
              ),
            ],
          ),
      ],
    );
