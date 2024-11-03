using System;
using System.Numerics;
using FyroxLite;

public class Player : NodeScript
{
    public float sensitivity;
    public Node camera;
    public float power;
    public Prefab bullet;
    public float initialBulletVelocity;
    public float shootingRange;
    public float reloadDelaySec;
    private float reloadSec;
    private bool published;
    private Node collider;
    private float aimY;

    private const int FRACTION_PLAYER = 0;

    public void Turn(float x)
    {
        Quaternion rotDelta = Quaternion.FromAxisAngle(Vector3.Up, sensitivity * x);
        Node rotationNode = GetNode<Node>(".."); // Assuming the Player is a child of the rotation node
        rotationNode.Rotation = rotationNode.Rotation * rotDelta;
    }

    public void Aim(float y)
    {
        aimY += y * sensitivity;
        aimY = Mathf.Clamp(aimY, -Mathf.Pi / 2.0f, Mathf.Pi / 2.0f);
        camera.Rotation = Quaternion.FromAxisAngle(Vector3.Right, aimY);
    }

    public void Fire()
    {
        Vector3 cameraPos = camera.GlobalPosition;
        Quaternion bulletOrientation = camera.GlobalRotation;

        Bullet.Spawn(new BulletSpawnParams
        {
            prefab = bullet,
            origin = cameraPos,
            direction = bulletOrientation * Vector3.Forward,
            initialVelocity = initialBulletVelocity,
            authorCollider = collider,
            range = shootingRange,
            fraction = FRACTION_PLAYER,
        });
    }

    public override void _Ready()
    {
        Input.SetMouseMode(Input.MouseMode.Captured);
        collider = GetNode<Node>("..").GetChild(0); // Update the path as necessary
        if (collider == null)
        {
            GD.PrintErr("Player collider missing");
        }
    }

    public override void _Process(float delta)
    {
        if (reloadSec > 0.0f)
        {
            reloadSec -= delta;
        }
        if (!published)
        {
            published = true;
            Game.Instance.Player = GetNode<Node>(".."); // Adjust as necessary
        }

        if (Input.IsActionPressed("Fire"))
        {
            if (reloadSec <= 0.0f)
            {
                reloadSec = reloadDelaySec;
                Fire();
            }
        }

        Vector3 moveDelta = Vector3.Zero;

        if (Input.IsKey(KeyCode.W))
        {
            moveDelta.Z += 1.0f;
        }
        if (Input.IsKey(KeyCode.S))
        {
            moveDelta.Z -= 1.0f;
        }
        if (Input.IsKey(KeyCode.A))
        {
            moveDelta.X -= 1.0f;
        }
        if (Input.IsKey(KeyCode.D))
        {
            moveDelta.X += 1.0f;
        }

        Turn(-Input.MouseMove.X);
        Aim(Input.MouseMove.Y);

        if (moveDelta.LengthSquared() > 0.001f)
        {
            moveDelta = moveDelta.Normalized();
        }

        // Assuming the Player is a RigidBody or has a Rigidbody component
        RigidBody body = GetNode<RigidBody>(".."); // Adjust path as necessary
        if (body != null)
        {
            Vector3 selfRotation = body.GlobalRotation * moveDelta;
            Vector3 force = selfRotation * power;
            body.AddForce(force);
        }
    }

    public void OnMessage(object message)
    {
        if (message is BulletHitMessage hit && hit.Fraction != FRACTION_PLAYER)
        {
            Game.Instance.IncWounds();
            GD.Print("Player wounded!");
        }
    }
}
