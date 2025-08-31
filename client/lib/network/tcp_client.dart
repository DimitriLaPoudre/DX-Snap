import 'dart:io';
import 'dart:convert';

import 'package:web_socket_channel/web_socket_channel.dart';

import '../tools.dart';

class TcpClient {
  WebSocketChannel? _socket;

  Future<void> connect(String host, int port) async {
    debugLogs("before connection");
    _socket = WebSocketChannel.connect(Uri.parse("ws://$host:$port"));
    debugLogs("Connected to $host:$port");

    _socket?.stream.listen((data) {
      final message = data;
      debugLogs("Received from server: $message");
    }, onError: (err) {
      debugLogs("Socket error: $err");
      //_socket?.sink.close();
    }, onDone: () {
      debugLogs("Connection closed by server");
      //_socket?.sink.close();
    });
  }

  Future<void> sendJsonAuth(String type, String username, String password) async {
    if (_socket != null) {
      final data = jsonEncode({"type":type, "username":username, "password":password});
      _socket!.sink.add(data);
    }
  }

  void close() {
    _socket?.sink.close();
    _socket = null;
    debugLogs("Connection closed");
  }
}
