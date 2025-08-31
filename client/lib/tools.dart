import 'package:flutter/foundation.dart';

void debugLogs(String message) {
	if (kDebugMode) {
		debugPrint(message);
	}
}

