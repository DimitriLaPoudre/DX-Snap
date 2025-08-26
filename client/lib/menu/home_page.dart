import 'package:flutter/material.dart';

import '../network/tcp_client.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final TcpClient _tcpClient = TcpClient();

  @override
  void initState() {
    super.initState();
    connectToServer();
  }

  void connectToServer() async {
    try {
      await _tcpClient.connect('127.0.0.1', 8080);
    } catch (e) {
      debugPrint("Error connecting to server: $e");
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('DX Snap Home Page'),
      ),
      body: const Center(
        child: Text('Welcome to DX Snap!'),
      ),
    );
  }
}
