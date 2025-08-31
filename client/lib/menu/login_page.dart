import 'package:flutter/material.dart';

import '../network/tcp_client.dart';
import '../tools.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  State<LoginPage> createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  
  bool _isLogin = true;
  final usernameController = TextEditingController();
  final passwordController = TextEditingController();
  TcpClient? _tcpClient;

  @override
  void initState() {
    super.initState();
    _tcpClient = TcpClient();
    connectToServer(_tcpClient!);
  }

  @override
  void dispose() {
    usernameController.dispose();
    passwordController.dispose();
    _tcpClient?.close();
    super.dispose();
  }

  void connectToServer(TcpClient client) async {
    try {
      client.connect('192.168.1.17', 13216);
    } catch (e) {
      debugLogs("Error connecting to server: $e");
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Authentication")),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ElevatedButton(
                  onPressed: () {
                    setState(() => _isLogin = true);
                  },
                  child: const Text("Login"),
                ),
                const SizedBox(width: 12),
                ElevatedButton(
                  onPressed: () {
                    setState(() => _isLogin = false);
                  },
                  child: const Text("Register"),
                ),
              ],
            ),
            const SizedBox(height: 20),

            _isLogin ? _buildLoginForm() : _buildRegisterForm(),
          ],
        ),
      ),
    );
  }

  Widget _buildLoginForm() {
    return Column(
      children: [
        TextField(
          controller: usernameController,
          decoration: InputDecoration(
            labelText: "Username",
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 12),
        TextField(
          controller: passwordController,
          obscureText: true,
          decoration: InputDecoration(
            labelText: "Password",
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: () {
            if (usernameController.text.trim().isNotEmpty && passwordController.text.trim().isNotEmpty) {
              _tcpClient?.sendJsonAuth("Connect", usernameController.text, passwordController.text);
            }
          },
          child: const Text("Login"),
        ),
      ],
    );
  }

  Widget _buildRegisterForm() {
    return Column(
      children: [
        TextField(
          controller: usernameController,
          decoration: InputDecoration(
            labelText: "Username",
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 12),
        TextField(
          controller: passwordController,
          obscureText: true,
          decoration: InputDecoration(
            labelText: "Password",
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 20),
        ElevatedButton(
          onPressed: () {
            if (usernameController.text.trim().isNotEmpty && passwordController.text.trim().isNotEmpty) {
              _tcpClient?.sendJsonAuth("Create", usernameController.text, passwordController.text);
            }
          },
          child: const Text("Register"),
        ),
      ],
    );
  }
}