#pragma once
#include "stdafx.h"
#include <exception>

class GameException : std::exception
{
private:
	const wstring _what;

public:
	GameException(const wstring& what);
	~GameException();

	const wstring& what() const;
};
