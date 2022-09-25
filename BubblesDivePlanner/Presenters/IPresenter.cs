using System.Collections.Generic;
using BattleSimulator.Models.Mechs;
using BattleSimulator.Models.Weapons;

namespace BattleSimulator.Presenters
{
    public interface IPresenter
    {
        void DisplayMech(IMech mechToDisplay);
        IMech SelectMech(IList<IMech> mechs);
        IWeapon SelectWeapon(IList<IWeapon> weapons);
    }
}