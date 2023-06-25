import 'package:api/api.dart';
import 'package:hive/hive.dart';

class MovieAdapter extends TypeAdapter<Movie> {
  @override
  final typeId = 1;

  @override
  Movie read(BinaryReader reader) {
    var numOfFields = reader.readByte();
    var fields = <int, dynamic>{
      for (var i = 0; i < numOfFields; i++) reader.readByte(): reader.read(),
    };

    return Movie(
      description: fields[0] as String,
      genre: fields[1] as String,
      id: fields[2] as String,
      imdbLink: fields[3] as String?,
      length: fields[4] as double,
      name: fields[5] as String,
      releaseDate: fields[6] as DateTime,
    );
  }

  @override
  void write(BinaryWriter writer, Movie obj) {
    writer
      ..writeByte(7)
      ..writeByte(0)
      ..write(obj.description)
      ..writeByte(1)
      ..write(obj.genre)
      ..writeByte(2)
      ..write(obj.id)
      ..writeByte(3)
      ..write(obj.imdbLink)
      ..writeByte(4)
      ..write(obj.length)
      ..writeByte(5)
      ..write(obj.name)
      ..writeByte(6)
      ..write(obj.releaseDate);
  }
}
