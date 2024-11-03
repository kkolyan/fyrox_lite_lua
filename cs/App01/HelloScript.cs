using FyroxLite;

namespace App01;

[Uuid("41e64c51-ccb5-41e8-9a1b-cb50debe735f")]
public class HelloScript : NodeScript
{
    public void OnInit()
    {
        Console.WriteLine("Hello OnInit");
    }

    public void OnUpdate()
    {
        Console.WriteLine("Hello OnUpdate");
    }
}