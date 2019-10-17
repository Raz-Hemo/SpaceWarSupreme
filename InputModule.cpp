#include "stdafx.h"
#include "InputModule.h"
#include <Windowsx.h>

LRESULT CALLBACK InputModule::WndProc(HWND hwnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
	switch (msg)
	{
	case WM_DESTROY:
		PostQuitMessage(0);
		break;
	case WM_LBUTTONDOWN:
	case WM_LBUTTONUP:
		if (mousehook)
			mousehook(MouseEvent(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam), msg == WM_LBUTTONDOWN));
		break;
	case WM_SIZE:
		if (resizehook && wParam == SIZE_RESTORED)
			resizehook(LOWORD(lParam), HIWORD(lParam));
		break;
	default:
		return DefWindowProc(hwnd, msg, wParam, lParam);
	}
	return 0;
}
