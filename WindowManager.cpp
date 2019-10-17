#include "stdafx.h"
#include "WindowManager.h"
#include "InputModule.h"
#include "resource.h"

LRESULT CALLBACK WndProc(HWND hwnd,
	UINT msg,
	WPARAM wParam,
	LPARAM lParam)
{
	// we stored the reference to input in the window long pointer
	auto x = GetWindowLongPtr(hwnd, 0);

	InputModule* input = reinterpret_cast<InputModule*>(x);

	if (input)
		return input->WndProc(hwnd, msg, wParam, lParam);

	return DefWindowProc(hwnd, msg, wParam, lParam);
}

WindowManager::WindowManager(HINSTANCE hInstance, int nCmdShow, InputModule& _input) : input(_input)
{
	WNDCLASSEX wc;

	HICON icon = (HICON)LoadImage(hInstance,
		MAKEINTRESOURCE(IDI_ICON1),
		IMAGE_ICON,
		LR_DEFAULTSIZE, LR_DEFAULTSIZE,
		LR_DEFAULTCOLOR);

	wc.cbSize = sizeof(WNDCLASSEX);
	wc.hInstance = hInstance;
	wc.lpszClassName = Constants::ProgramName.c_str();
	wc.style = 0;
	wc.lpfnWndProc = WndProc;
	wc.hbrBackground = (HBRUSH)GetStockObject(BLACK_BRUSH);
	wc.lpszMenuName = NULL;
	wc.hIcon = icon;
	wc.hIconSm = icon;
	wc.hCursor = LoadCursor(NULL, IDC_ARROW);
	wc.cbClsExtra = NULL;
	wc.cbWndExtra = sizeof(InputModule*);

	if (!RegisterClassEx(&wc))
		FatalError("Failed registering the window class");

	RECT rect;
	rect.left = 0; rect.top = 0;
	rect.right = Constants::WindowWidth; rect.bottom = Constants::WindowHeight;
	AdjustWindowRect(&rect, WS_OVERLAPPEDWINDOW, FALSE);
	hwnd = CreateWindowEx(
		NULL,
		Constants::ProgramName.c_str(),
		Constants::ProgramName.c_str(),
		WS_OVERLAPPEDWINDOW,
		CW_USEDEFAULT, CW_USEDEFAULT,
		rect.right - rect.left,
		rect.bottom - rect.top,
		NULL,
		NULL,
		hInstance,
		NULL
	);

	if (!hwnd)
		FatalError("Failed creating window");

	ShowWindow(hwnd, nCmdShow); 
	UpdateWindow(hwnd);

	SetWindowLongPtr(hwnd, 0, reinterpret_cast<LONG_PTR>(&input));
}

WindowManager::~WindowManager()
{
	DestroyWindow(hwnd);
}

HWND WindowManager::WindowHandle() const
{
	return hwnd;
}

bool WindowManager::IsForeground() const
{
	return GetForegroundWindow() == hwnd;
}