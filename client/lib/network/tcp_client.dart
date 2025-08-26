import 'dart:io';
import 'dart:convert';

import '../tools.dart';

class TcpClient {
  Socket? _socket;

  

  Future<void> connect(String host, int port) async {
    _socket = await Socket.connect(host, port);
    debugPrint("Connected to $host:$port");

    _socket!.listen((data) {
      final message = utf8.decode(data);
      debugPrint("Received from server: $message");
    }, onError: (err) {
      debugPrint("Socket error: $err");
      _socket?.destroy();
    }, onDone: () {
      debugPrint("Connection closed by server");
      _socket?.destroy();
    });
  }

  void send(String message) {
    if (_socket != null) {
      _socket!.write(message);
      debugPrint("Sent: $message");
    } else {
      debugPrint("Error sending message");
    }
  }

  void sendJson(Map<String, dynamic> json) {
    if (_socket != null) {
      final message = jsonEncode(json);
      _socket!.writeln(message);
      debugPrint("Sent JSON: $message");
    } else {
      debugPrint("Error sending JSON");
    }
  }

  void close() {
    _socket?.close();
    _socket = null;
    debugPrint("Connection closed");
  }
}