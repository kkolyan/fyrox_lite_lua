using FyroxLite;

[Uuid("7c259fd2-fdb9-453b-a7ef-19cdd85428cc")]
public class Beacon : NodeScript
{
    protected override void OnUpdate(float dt)
    {
        Vector3 pos = Node.GlobalPosition;

        Plugin.Get<Game>().beacons.Add(pos);

        Console.WriteLine($"Beacon registered: {Node}");

        Node.Destroy();
    }
}