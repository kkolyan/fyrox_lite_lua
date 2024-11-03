using FyroxLite;

[Uuid("c5671d19-9f1a-4286-8486-add4ebaadaec")]
public class Player: NodeScript
{
    private float Sensitivity;
    private Node Camera;
    private float Power;
    private Prefab Bullet;
    private float InitialBulletVelocity;
    private float ShootingRange;
    private float ReloadDelaySec;
    
    [HideInInspector]
    [Transient]
    private float ReloadSec;
    
    [HideInInspector]
    [Transient]
    private bool Published;
    
    [HideInInspector]
    [Transient]
    private Node Collider;
    
    [HideInInspector]
    [Transient]
    private float AimY;

    private const int FractionPlayer = 0;

    public void Turn(float x)
    {
        Quaternion rotDelta = Quaternion.FromEuler(Vector3.Up * Sensitivity * x);
        Node.LocalRotation *= rotDelta;
    }

    public void Aim(float y)
    {
        AimY += y * Sensitivity;
        AimY = Math.Max(-MathF.PI / 2.0f, Math.Min(AimY, MathF.PI / 2.0f));

        Camera.LocalRotation = Quaternion.FromEuler(Vector3.Right * AimY);
    }

    public void Fire()
    {
        Vector3 cameraPos = Camera.GlobalPosition;
        Quaternion bulletOrientation = Camera.GlobalRotation;

        global::Bullet.Spawn(new Bullet.BulletSeed
        {
            Prefab = Bullet,
            Origin = cameraPos,
            Direction = bulletOrientation * Vector3.Forward,
            InitialVelocity = InitialBulletVelocity,
            AuthorCollider = Collider,
            Range = ShootingRange,
            Fraction = FractionPlayer,
        });
    }

    protected override void OnInit()
    {
        Window.CursorGrab = CursorGrabMode.Confined;
        Collider = Node.FindColliderInChildren() ?? throw new Exception("player collider missing");
    }

    protected override void OnStart()
    {
        Node.SubscribeTo();
    }

    protected override void OnMessage(object message)
    {
        if (message is BulletHitMessage hitMessage && hitMessage.Fraction != FractionPlayer)
        {
            Plugin.Get<Game>().IncWounds();
            Console.WriteLine("player wounded!");
        }
    }

    protected override void OnUpdate(float dt)
    {
        if (ReloadSec > 0.0f)
        {
            ReloadSec -= dt;
        }
        if (!Published)
        {
            Published = true;
            Plugin.Get<Game>().Player = Node;
        }

        if (Input.IsMouseButton(Input.MouseLeft))
        {
            if (ReloadSec <= 0.0f)
            {
                ReloadSec = ReloadDelaySec;
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
            moveDelta.X += 1.0f;
        }
        if (Input.IsKey(KeyCode.D))
        {
            moveDelta.X -= 1.0f;
        }

        Turn(-Input.MouseMove.X);
        Aim(Input.MouseMove.Y);

        if (moveDelta.Length() > 0.001f)
        {
            moveDelta = moveDelta.Normalized();
        }

        var selfRotation = Node.LocalRotation;
        Vector3 moveDirection = selfRotation * moveDelta;
        Vector3 force = moveDirection * Power;
        Node.AsRigidBody().Value.ApplyForce(force);
    }
}
