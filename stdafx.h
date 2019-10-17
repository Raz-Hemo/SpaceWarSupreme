#pragma once
#define WIN32_LEAN_AND_MEAN
#define NOMINMAX
#include<Windows.h>
#include<d3d11.h>
#include<d3dcompiler.h>
#include<exception>
#include<algorithm>
#include<iterator>
#include<functional>
#include<fstream>
#include<string>
#include<memory>
#include<map>
#include<unordered_map>
#include<list>
#include<vector>
#include<time.h>

using std::shared_ptr;
using std::make_shared;
using std::string;
using std::wstring;
using std::list;
using std::vector;
using std::exception;
using std::map;
using std::unordered_map;
using std::fstream;
using std::wfstream;
using std::ifstream;
using std::ofstream;
using std::wifstream;
using std::wofstream;

#pragma comment(lib, "d3d11.lib")
#pragma comment(lib, "dxgi.lib")
#pragma comment(lib, "d3dcompiler.lib")

namespace Constants
{
	const string ProgramName = "Space War Supreme!";
	const string ConfigPath = "config.txt";

	constexpr float Tau = 6.283185f;

	/* Window size */
	constexpr int WindowWidth = 800;
	constexpr int WindowHeight = 600;
}

void FatalError(const string& error);
bool isDebug();
