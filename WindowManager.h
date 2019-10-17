#pragma once

class InputModule;
class WindowManager
{
	HWND hwnd;
	InputModule& input;
public:
	WindowManager(HINSTANCE hInstance, int nCmdShow, InputModule& _input);
	~WindowManager();

	HWND WindowHandle() const;

	bool IsForeground() const;
};

