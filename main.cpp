#include "stdafx.h"
#include "GameException.h"
#include "WindowManager.h"
#include "Renderer.h"
#include "InputModule.h"

int WINAPI WinMain(HINSTANCE hinstance, HINSTANCE hPrevInstance,
	LPSTR lpCmdLine, int nCmdShow)
{
	UNREFERENCED_PARAMETER(lpCmdLine);
	UNREFERENCED_PARAMETER(hPrevInstance);

	MSG msg = { 0 };
	srand((unsigned int)time(NULL));

	try
	{
		InputModule input;
		WindowManager window(hinstance, nCmdShow, input);
		Renderer renderer(window.WindowHandle());
		GUI gui;

		// Handle input
		input.kbhook = [&](const Constants::Hotkeys& hk) { soundplayer.HandleEvent(hk); };
		input.mousehook = [&](const MouseEvent& e) { gui.HandleEvent(e); };
		input.resizehook = [&](int x, int y) {
			renderer.HandleResize(x, y);
		};

		while (true)
		{
			// Windows messages and input
			if (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE))
			{
				if (msg.message == WM_QUIT)
					break;

				TranslateMessage(&msg);
				DispatchMessage(&msg);
			}
			else
			{
				renderer.Frame();
			}
		}
	}
	catch (const GameException& e)
	{
		MessageBoxW(
			NULL,
			e.what().c_str(),
			L"Fatal error",
			MB_OK | MB_ICONERROR | MB_SERVICE_NOTIFICATION);
		return 1;
	}

	return 0;
}