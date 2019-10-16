#include "stdafx.h"
#include "GameException.h"

void FatalError(const wstring& error)
{
	throw GameException(error);
}

bool isDebug()
{
#ifdef _DEBUG
	return true;
#else
	return false;
#endif
}