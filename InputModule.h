#pragma once

struct MouseEvent
{
	int x, y;
	bool mousedown; // true if event is click, false if release.

	MouseEvent(int _x, int _y, bool _mousedown) : x(_x), y(_y), mousedown(_mousedown) {}
};

struct KeyboardEvent
{
	enum Keys
	{

	} key;
	bool keydown; // true if down, false if release.

	KeyboardEvent(Keys _key, bool _keydown) : key(_key), keydown(_keydown) {}
};

class InputModule
{
public:
	LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam);

	std::function<void(const KeyboardEvent&)> kbhook;
	std::function<void(const MouseEvent&)> mousehook;
	std::function<void(int, int)> resizehook;
};

