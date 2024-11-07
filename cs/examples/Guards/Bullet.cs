using System;
using System.Collections.Generic;
using FyroxLite;

[Uuid("12371d19-9f1a-4286-8486-add4ebaadaec")]
public class Bullet : NodeScript
{
    private Vector3 velocity;
    private float remaining_sec;
    private Node author_collider;
    private float fraction;

    public struct BulletSeed
    {
        public Prefab Prefab;
        public Vector3 Origin;
        public Vector3 Direction;
        public float InitialVelocity;
        public Node AuthorCollider;
        public float Range;
        public int Fraction;
    }

    public static void Spawn(BulletSeed seed)
    {
        Quaternion orientation = Basis.LookingAt(seed.Direction, Vector3.Up).GetRotationQuaternion();
        Node bullet = seed.Prefab.InstantiateAt(seed.Origin, orientation);
        Bullet script = bullet.FindScript<Bullet>();
        script.velocity = seed.Direction.Normalized() * seed.InitialVelocity;
        script.remaining_sec = seed.Range / seed.InitialVelocity;
        script.author_collider = seed.AuthorCollider;
        script.fraction = seed.Fraction;
    }

    protected override void OnUpdate(float deltaTime)
    {
        remaining_sec -= deltaTime;
        if (remaining_sec <= 0.0f)
        {
            Node.Destroy();
            return;
        }

        Vector3 newPos = Node.LocalPosition + velocity * deltaTime;

        RayCastOptions opts = new RayCastOptions
        {
            RayOrigin = Node.LocalPosition,
            RayDirection = velocity.Normalized(),
            MaxLen = velocity.Length() * deltaTime,
            SortResults = true
        };

        List<Intersection> results = Physics.CastRay(opts);

        foreach (var hit in results)
        {
            if (hit.Collider != author_collider)
            {
                hit.Collider.SendHierarchical(RoutingStrategy.Up, new BulletHitMessage { Fraction = (int)fraction });
                Node.Destroy();
                return;
            }
        }

        Node.LocalPosition = newPos;
    }
}