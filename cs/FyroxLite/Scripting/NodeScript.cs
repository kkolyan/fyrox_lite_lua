using Exception = System.Exception;

namespace FyroxLite;

public abstract class NodeScript
{
    public ref Node Node => throw new Exception();

    protected internal virtual void OnInit()
    {
    }

    protected internal virtual void OnStart()
    {
    }

    protected internal virtual void OnUpdate(float dt)
    {
    }

    protected internal virtual void OnMessage(object message)
    {
    }

    protected internal virtual void OnDeinit()
    {
    }
}