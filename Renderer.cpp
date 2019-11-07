#include "stdafx.h"
#include "Renderer.h"
#include "TextureResource.h"
#include "VertexShader.h"
#include "PixelShader.h"

Renderer::Renderer(HWND hwnd)
{
	DXGI_SWAP_CHAIN_DESC scd = { 0 };
	scd.BufferCount = 1;
	scd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
	scd.BufferDesc.Height = Constants::WindowHeight;
	scd.BufferDesc.Width = Constants::WindowWidth;
	scd.BufferDesc.RefreshRate.Numerator = 60;
	scd.BufferDesc.RefreshRate.Denominator = 1;
	scd.BufferDesc.Scaling = DXGI_MODE_SCALING_STRETCHED;
	scd.BufferDesc.ScanlineOrdering = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
	scd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
	scd.OutputWindow = hwnd;
	scd.Windowed = TRUE;
	scd.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
	scd.SampleDesc.Count = 1;
	scd.SampleDesc.Quality = 0;

	if (FAILED(D3D11CreateDeviceAndSwapChain(NULL, D3D_DRIVER_TYPE_HARDWARE, NULL,
		isDebug() ? D3D11_CREATE_DEVICE_DEBUG : NULL,
		NULL, NULL, D3D11_SDK_VERSION, &scd, &swapchain, &device, NULL, &devcon)))
	{
		FatalError("Failed creating directx device");
	}

	HandleResize(Constants::WindowWidth, Constants::WindowHeight);

	D3D11_BLEND_DESC blendDesc;
	blendDesc.RenderTarget[0].BlendEnable = true;
	blendDesc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
	blendDesc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
	blendDesc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
	blendDesc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
	blendDesc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ZERO;
	blendDesc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
	blendDesc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL;
	blendDesc.IndependentBlendEnable = false;
	blendDesc.AlphaToCoverageEnable = false;
	device->CreateBlendState(&blendDesc, &transparent_blend_state);
}

Renderer::~Renderer()
{
}

void Renderer::Frame()
{
	// Initialize
	FLOAT color[] = { 0.0,0.0,0.0,1.0 };
	devcon->ClearRenderTargetView(rtv, color);
	devcon->ClearDepthStencilView(dsv, D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL, 1.0f, 0);

	swapchain->Present(0, 0);
}

void Renderer::HandleResize(int x, int y)
{
	if (rtv) {rtv->Release(); rtv = nullptr;}
	if (dsv) {dsv->Release(); dsv = nullptr;}
	if (dsb) {dsb->Release(); dsb = nullptr;}

	D3D11_VIEWPORT vp = { 0 };
	vp.TopLeftX = 0;
	vp.TopLeftY = 0;
	vp.Width = float(x);
	vp.Height = float(y);
	vp.MaxDepth = 1.0f;
	vp.MinDepth = 0.0f;
	devcon->RSSetViewports(1, &vp);

	DXGI_MODE_DESC new_mode;
	new_mode.Width = x;
	new_mode.Height = y;
	new_mode.Scaling = DXGI_MODE_SCALING_STRETCHED;
	new_mode.ScanlineOrdering = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;
	new_mode.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
	new_mode.RefreshRate.Numerator = 60;
	new_mode.RefreshRate.Denominator = 1;
	swapchain->ResizeTarget(&new_mode);

	D3D11_TEXTURE2D_DESC depthStencilDesc;
	depthStencilDesc.Width = x;
	depthStencilDesc.Height = y;
	depthStencilDesc.MipLevels = 1;
	depthStencilDesc.ArraySize = 1;
	depthStencilDesc.Format = DXGI_FORMAT_D24_UNORM_S8_UINT;
	depthStencilDesc.SampleDesc.Count = 1;
	depthStencilDesc.SampleDesc.Quality = 0;
	depthStencilDesc.Usage = D3D11_USAGE_DEFAULT;
	depthStencilDesc.BindFlags = D3D11_BIND_DEPTH_STENCIL;
	depthStencilDesc.CPUAccessFlags = 0;
	depthStencilDesc.MiscFlags = 0;
	device->CreateTexture2D(&depthStencilDesc, NULL, &dsb);
	device->CreateDepthStencilView(dsb, NULL, &dsv);

	swapchain->ResizeBuffers(1, x, y, new_mode.Format, 0);

	ID3D11Texture2D* BackBuffer;
	swapchain->GetBuffer(0, __uuidof(ID3D11Texture2D), (void**)&BackBuffer);
	device->CreateRenderTargetView(BackBuffer, NULL, &rtv);
	devcon->OMSetRenderTargets(1, &rtv, dsv);
	BackBuffer->Release();

	client_x = float(x);
	client_y = float(y);
}
