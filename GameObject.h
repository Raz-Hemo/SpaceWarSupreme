#pragma once
#include "stdafx.h"

class Renderable;
class GameObject
{
private:
	DirectX::XMFLOAT4X4 transform;
	DirectX::XMFLOAT3 velocity;
	list<shared_ptr<GameObject>> children;
	shared_ptr<GameObject> parent;
	shared_ptr<Renderable> renderable;

public:
	GameObject();
	virtual ~GameObject();

	void AddChild(shared_ptr<GameObject> child);
	void Detach(shared_ptr<GameObject> child);

	const list<shared_ptr<GameObject>>& GetChildren() const;

	void SetRenderable(shared_ptr<Renderable> r);
};