// SPDX-License-Identifier: None
pragma solidity ^0.8.19;

struct CollInfo {
    address[] pools;
    uint256[] shares;
    uint256[] amts;
    address[] wLps;
    uint256[][] ids;
    uint256[][] wLpAmts;
}

struct BorrInfo {
    address[] pools;
    uint256[] debtShares;
    uint256[] debts;
}

struct PosInfo {
    CollInfo collInfo;
    BorrInfo borrInfo;
    uint256 collCredit_e36;
    uint256 borrCredit_e36;
    uint256 health_e18;
    uint16 mode;
    address viewer;
    address owner;
    uint256 posId; // nft id
}

/// @title Init Lens Interface
interface IInitLens {
    /// @dev get hook's user position info
    /// @param _hook hook address
    /// @param _user user address
    /// @param _posId hook's position id
    /// @return posInfo position info
    function getHookPosInfo(address _hook, address _user, uint256 _posId) external returns (PosInfo memory posInfo);

    /// @dev get hook's user position infos
    /// @param _hook hook address
    /// @param _user user address
    /// @param _posIds hook's position ids
    /// @return posInfos position infos
    function getHookPosInfos(address _hook, address _user, uint256[] calldata _posIds)
        external
        returns (PosInfo[] memory posInfos);

    /// @dev get init position info
    /// @param _initPosId init position id
    /// @return posInfo position info
    function getInitPosInfo(uint256 _initPosId) external returns (PosInfo memory posInfo);

    /// @dev get init position infos
    /// @param _initPosIds init position ids
    /// @return posInfos position infos
    function getInitPosInfos(uint256[] calldata _initPosIds) external returns (PosInfo[] memory posInfos);

    /// @dev get mode's pool borrowable amount
    /// @param _mode mode
    /// @param _pool pool address
    /// @return borrowableAmt borrowable amount for the pool of the mode
    function modeBorrowableAmt(uint16 _mode, address _pool) external returns (uint256 borrowableAmt);

    /// @dev get position's pool borrowable amount
    /// @param _hook hook address
    /// @param _user user address
    /// @param _posId hook's position id
    /// @param _pool pool address
    /// @return borrowableAmt borrowable amount for the pool of the position
    function posBorrowableAmt(address _hook, address _user, uint256 _posId, address _pool)
        external
        returns (uint256 borrowableAmt);

    /// @dev get position info from viewer with index
    /// @param _viewer position viewer
    /// @param _index viewer's index
    /// @return posInfo position info
    function viewerPosInfoAt(address _viewer, uint256 _index) external returns (PosInfo memory posInfo);

    /// @dev get list of position info from viewer with list of indices
    /// @param _viewer position viewer
    /// @param _indices list of viewer's indices
    /// @return posInfos list of position info
    function viewerPosInfos(address _viewer, uint256[] calldata _indices)
        external
        returns (PosInfo[] memory posInfos);
}
