#pragma once
#include "stdafx.h"

class VertexShader;
class PixelShader;
class TextureResource;

class Renderer
{
	ID3D11Device* device;
	ID3D11DeviceContext* devcon;
	IDXGISwapChain* swapchain;

	ID3D11RenderTargetView* rtv;
	ID3D11DepthStencilView* dsv;
	ID3D11Texture2D* dsb;
	ID3D11BlendState* transparent_blend_state;
	float client_x;
	float client_y;

	shared_ptr<PixelShader> visualizer_ps;
	shared_ptr<VertexShader> font_vs;
	shared_ptr<PixelShader> font_ps;

	unordered_map<wstring, shared_ptr<TextureResource>> textures;
public:
	Renderer(HWND hwnd);
	~Renderer();

	void Frame();

	void HandleResize(int x, int y);
};

