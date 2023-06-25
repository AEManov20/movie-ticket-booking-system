import 'package:api/api.dart';
import 'package:hive/hive.dart';

class TheatreAdapter extends TypeAdapter<Theatre> {
  @override
  final typeId = 2;

  @override
  Theatre read(BinaryReader reader) {
    var numOfFields = reader.readByte();
    var fields = <int, dynamic>{
      for (var i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };

    return Theatre(
      id: fields[0] as String,
      locationLat: fields[0] as double,
      locationLon: fields[0] as double,
      name: fields[0] as String,
    );
  }

  @override
  void write(BinaryWriter writer, Theatre obj) {
    writer
      ..writeByte(4)
      ..writeByte(0)
      ..write(obj.id)
      ..writeByte(1)
      ..write(obj.locationLat)
      ..writeByte(2)
      ..write(obj.locationLon)
      ..writeByte(3)
      ..write(obj.name);
  }
}
