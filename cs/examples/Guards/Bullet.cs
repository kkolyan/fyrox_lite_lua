using System;
using System.Collections.Generic;
using FyroxLite;

[Uuid("12371d19-9f1a-4286-8486-add4ebaadaec")]
public class Bullet : NodeScript
{
    private Vector3 Velocity;
    private float RemainingSeconds;
    private Node AuthorCollider;
    private int Fraction;

    public class BulletSeed
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
        script.Velocity = seed.Direction.Normalized() * seed.InitialVelocity;
        script.RemainingSeconds = seed.Range / seed.InitialVelocity;
        script.AuthorCollider = seed.AuthorCollider;
        script.Fraction = seed.Fraction;
    }

    protected override void OnUpdate(float deltaTime)
    {
        RemainingSeconds -= deltaTime;
        if (RemainingSeconds <= 0.0f)
        {
            Node.Destroy();
            return;
        }

        Vector3 newPos = Node.LocalPosition + Velocity * deltaTime;

        RayCastOptions opts = new RayCastOptions
        {
            RayOrigin = Node.LocalPosition,
            RayDirection = Velocity.Normalized(),
            MaxLen = Velocity.Length() * deltaTime,
            SortResults = true
        };

        List<Intersection> results = Physics.CastRay(opts);

        foreach (var hit in results)
        {
            if (hit.Collider != AuthorCollider)
            {
                hit.Collider.SendHierarchical(RoutingStrategy.Up, new BulletHitMessage { Fraction = Fraction });
                Node.Destroy();
                return;
            }
        }

        Node.LocalPosition = newPos;
    }
}