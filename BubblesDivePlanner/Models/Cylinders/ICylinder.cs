namespace BubblesDivePlanner.Models.Cylinders
{
    public interface ICylinder
    {
        byte CylinderVolume { get; }
        ushort CylinderPressure { get; }
        ushort InitialPressurisedVolume { get; }
        IGasMixture GasMixture { get; }
        IGasManagement GasManagement { get; }
    }
}