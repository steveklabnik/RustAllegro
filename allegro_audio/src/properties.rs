// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use ffi::*;

#[deriving(PartialEq)]
pub enum AudioDepth
{
	AudioDepthI8,
	AudioDepthI16,
	AudioDepthI24,
	AudioDepthU8,
	AudioDepthU16,
	AudioDepthU24,
	AudioDepthF32,
}

impl AudioDepth
{
	pub fn from_allegro(val: ALLEGRO_AUDIO_DEPTH) -> AudioDepth
	{
		match val
		{
			ALLEGRO_AUDIO_DEPTH_INT8     => AudioDepthI8,
			ALLEGRO_AUDIO_DEPTH_INT16    => AudioDepthI16,
			ALLEGRO_AUDIO_DEPTH_INT24    => AudioDepthI24,
			ALLEGRO_AUDIO_DEPTH_UNSIGNED => AudioDepthU8,
			ALLEGRO_AUDIO_DEPTH_UINT16   => AudioDepthU16,
			ALLEGRO_AUDIO_DEPTH_UINT24   => AudioDepthU24,
			ALLEGRO_AUDIO_DEPTH_FLOAT32  => AudioDepthF32,
		}
	}

	pub fn get(&self) -> ALLEGRO_AUDIO_DEPTH
	{
		match *self
		{
			AudioDepthI8  => ALLEGRO_AUDIO_DEPTH_INT8,
			AudioDepthI16 => ALLEGRO_AUDIO_DEPTH_INT16,
			AudioDepthI24 => ALLEGRO_AUDIO_DEPTH_INT24,
			AudioDepthU8  => ALLEGRO_AUDIO_DEPTH_UNSIGNED,
			AudioDepthU16 => ALLEGRO_AUDIO_DEPTH_UINT16,
			AudioDepthU24 => ALLEGRO_AUDIO_DEPTH_UINT24,
			AudioDepthF32 => ALLEGRO_AUDIO_DEPTH_FLOAT32,
		}
	}

	pub fn get_byte_size(&self) -> uint
	{
		match *self
		{
			AudioDepthI8  => 1,
			AudioDepthI16 => 2,
			AudioDepthI24 => 3,
			AudioDepthU8  => 1,
			AudioDepthU16 => 2,
			AudioDepthU24 => 3,
			AudioDepthF32 => 4,
		}
	}
}

pub enum ChannelConf
{
	ChannelConf1,
	ChannelConf2,
	ChannelConf3,
	ChannelConf4,
	ChannelConf51,
	ChannelConf61,
	ChannelConf71,
}

impl ChannelConf
{
	pub fn from_allegro(val: ALLEGRO_CHANNEL_CONF) -> ChannelConf
	{
		match val
		{
			ALLEGRO_CHANNEL_CONF_1   => ChannelConf1,
			ALLEGRO_CHANNEL_CONF_2   => ChannelConf2,
			ALLEGRO_CHANNEL_CONF_3   => ChannelConf3,
			ALLEGRO_CHANNEL_CONF_4   => ChannelConf4,
			ALLEGRO_CHANNEL_CONF_5_1 => ChannelConf51,
			ALLEGRO_CHANNEL_CONF_6_1 => ChannelConf61,
			ALLEGRO_CHANNEL_CONF_7_1 => ChannelConf71,
		}
	}

	pub fn get(&self) -> ALLEGRO_CHANNEL_CONF
	{
		match *self
		{
			ChannelConf1  => ALLEGRO_CHANNEL_CONF_1,
			ChannelConf2  => ALLEGRO_CHANNEL_CONF_2,
			ChannelConf3  => ALLEGRO_CHANNEL_CONF_3,
			ChannelConf4  => ALLEGRO_CHANNEL_CONF_4,
			ChannelConf51 => ALLEGRO_CHANNEL_CONF_5_1,
			ChannelConf61 => ALLEGRO_CHANNEL_CONF_6_1,
			ChannelConf71 => ALLEGRO_CHANNEL_CONF_7_1,
		}
	}

	pub fn get_num_channels(&self) -> uint
	{
		match *self
		{
			ChannelConf1  => 1,
			ChannelConf2  => 2,
			ChannelConf3  => 3,
			ChannelConf4  => 4,
			ChannelConf51 => 6,
			ChannelConf61 => 7,
			ChannelConf71 => 8,
		}
	}
}

pub enum Playmode
{
	PlaymodeOnce,
	PlaymodeLoop,
	PlaymodeBiDir,
}

impl Playmode
{
	pub fn from_allegro(val: ALLEGRO_PLAYMODE) -> Playmode
	{
		match val
		{
			ALLEGRO_PLAYMODE_ONCE => PlaymodeOnce,
			ALLEGRO_PLAYMODE_LOOP => PlaymodeLoop,
			ALLEGRO_PLAYMODE_BIDIR => PlaymodeBiDir,
			_ => unreachable!(),
		}
	}

	pub fn get(&self) -> ALLEGRO_PLAYMODE
	{
		match *self
		{
			PlaymodeOnce => ALLEGRO_PLAYMODE_ONCE,
			PlaymodeLoop => ALLEGRO_PLAYMODE_LOOP,
			PlaymodeBiDir => ALLEGRO_PLAYMODE_BIDIR,
		}
	}
}

pub enum MixerQuality
{
	MixerQualityPoint,
	MixerQualityLinear,
	MixerQualityCubic,
}

impl MixerQuality
{
	pub fn from_allegro(val: ALLEGRO_MIXER_QUALITY) -> MixerQuality
	{
		match val
		{
			ALLEGRO_MIXER_QUALITY_POINT  => MixerQualityPoint,
			ALLEGRO_MIXER_QUALITY_LINEAR => MixerQualityLinear,
			ALLEGRO_MIXER_QUALITY_CUBIC  => MixerQualityCubic,
		}
	}

	pub fn get(&self) -> ALLEGRO_MIXER_QUALITY
	{
		match *self
		{
			MixerQualityPoint => ALLEGRO_MIXER_QUALITY_POINT,
			MixerQualityLinear => ALLEGRO_MIXER_QUALITY_LINEAR,
			MixerQualityCubic => ALLEGRO_MIXER_QUALITY_CUBIC,
		}
	}
}
